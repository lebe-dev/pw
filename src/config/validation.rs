use ipnet::IpNet;
use log::warn;
use std::net::IpAddr;
use std::str::FromStr;
use thiserror::Error;

use super::model::{IpLimitEntry, IpLimitsConfig};

/// Validation errors for IP limits configuration
#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Invalid IP address format '{ip}': {reason}")]
    InvalidIpFormat { ip: String, reason: String },

    #[error("Invalid CIDR notation '{cidr}': {reason}")]
    InvalidCidrFormat { cidr: String, reason: String },

    #[error("Message max length {value} exceeds maximum allowed value of {max}")]
    MessageLengthTooHigh { value: u16, max: u16 },

    #[error("Message max length cannot be zero")]
    MessageLengthZero,

    #[error("File max size {value} exceeds maximum allowed value of {max} bytes")]
    FileSizeTooHigh { value: u64, max: u64 },

    #[error("File max size cannot be zero")]
    FileSizeZero,

    #[error("Empty IP string is not allowed")]
    EmptyIpString,

    #[error("Whitespace-only IP string is not allowed")]
    WhitespaceOnlyIp,

    #[error("IP address contains invalid characters: '{ip}'")]
    InvalidIpCharacters { ip: String },

    #[error("Duplicate IP entry found: '{ip}'")]
    DuplicateIpEntry { ip: String },

    #[error("IPv6 address '{ip}' is missing proper formatting")]
    Ipv6FormattingError { ip: String },

    #[error("CIDR prefix length {prefix} is invalid for {ip_type} address")]
    InvalidCidrPrefix { prefix: u8, ip_type: String },
}

/// Configuration validation limits
const MAX_MESSAGE_LENGTH: u16 = 65535; // Maximum u16 value
const MAX_FILE_SIZE: u64 = 10_737_418_240; // 10GB
const MIN_MESSAGE_LENGTH: u16 = 1;
const MIN_FILE_SIZE: u64 = 1;

/// Validates IP limits configuration
pub fn validate_ip_limits_config(config: &IpLimitsConfig) -> Result<(), Vec<ValidationError>> {
    let mut errors = Vec::new();

    if config.whitelist.is_empty() && config.enabled {
        warn!("ip whitelist is enabled but empty")
    }

    let mut seen_ips = std::collections::HashSet::new();

    for entry in &config.whitelist {
        if let Err(err) = validate_ip_entry(entry) {
            errors.extend(err);
        }

        if !seen_ips.insert(&entry.ip) {
            errors.push(ValidationError::DuplicateIpEntry {
                ip: entry.ip.clone(),
            });
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Validates a single IP limit entry
pub fn validate_ip_entry(entry: &IpLimitEntry) -> Result<(), Vec<ValidationError>> {
    let mut errors = Vec::new();

    if let Err(err) = validate_ip_format(&entry.ip) {
        errors.push(err);
    }

    if let Some(length) = entry.message_max_length {
        if let Err(err) = validate_message_length(length) {
            errors.push(err);
        }
    }

    if let Some(size) = entry.file_max_size {
        if let Err(err) = validate_file_size(size) {
            errors.push(err);
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Validates IP address format (supports IPv4, IPv6, and CIDR notation)
pub fn validate_ip_format(ip_str: &str) -> Result<(), ValidationError> {
    if ip_str.is_empty() {
        return Err(ValidationError::EmptyIpString);
    }

    if ip_str.trim().is_empty() {
        return Err(ValidationError::WhitespaceOnlyIp);
    }

    let trimmed_ip = ip_str.trim();

    if trimmed_ip
        .chars()
        .any(|c| !c.is_ascii_hexdigit() && !":./-".contains(c))
    {
        return Err(ValidationError::InvalidIpCharacters {
            ip: ip_str.to_string(),
        });
    }

    if trimmed_ip.contains('/') {
        validate_cidr_format(trimmed_ip)
    } else {
        validate_single_ip_format(trimmed_ip)
    }
}

/// Validates CIDR notation format
fn validate_cidr_format(cidr_str: &str) -> Result<(), ValidationError> {
    match IpNet::from_str(cidr_str) {
        Ok(network) => {
            let prefix_len = network.prefix_len();
            match network.addr() {
                IpAddr::V4(_) => {
                    if prefix_len > 32 {
                        return Err(ValidationError::InvalidCidrPrefix {
                            prefix: prefix_len,
                            ip_type: "IPv4".to_string(),
                        });
                    }
                }
                IpAddr::V6(_) => {
                    if prefix_len > 128 {
                        return Err(ValidationError::InvalidCidrPrefix {
                            prefix: prefix_len,
                            ip_type: "IPv6".to_string(),
                        });
                    }
                }
            }
            Ok(())
        }
        Err(e) => Err(ValidationError::InvalidCidrFormat {
            cidr: cidr_str.to_string(),
            reason: e.to_string(),
        }),
    }
}

/// Validates single IP address format
fn validate_single_ip_format(ip_str: &str) -> Result<(), ValidationError> {
    match IpAddr::from_str(ip_str) {
        Ok(_) => Ok(()),
        Err(e) => {
            let reason = if ip_str.contains(':') {
                if ip_str.len() < 3 || !ip_str.contains("::") && ip_str.split(':').count() != 8 {
                    "IPv6 address format is incorrect".to_string()
                } else {
                    format!("IPv6 parsing failed: {}", e)
                }
            } else if ip_str.contains('.') {
                if ip_str.split('.').count() != 4 {
                    "IPv4 address must have exactly 4 octets".to_string()
                } else if ip_str
                    .split('.')
                    .any(|octet| octet.parse::<u32>().map_or(true, |n| n > 255))
                {
                    "IPv4 address octets must be between 0 and 255".to_string()
                } else {
                    format!("IPv4 parsing failed: {}", e)
                }
            } else {
                format!("Unknown IP format: {}", e)
            };

            Err(ValidationError::InvalidIpFormat {
                ip: ip_str.to_string(),
                reason,
            })
        }
    }
}

/// Validates message length limits
pub fn validate_message_length(length: u16) -> Result<(), ValidationError> {
    if length == 0 {
        return Err(ValidationError::MessageLengthZero);
    }

    if length > MAX_MESSAGE_LENGTH {
        return Err(ValidationError::MessageLengthTooHigh {
            value: length,
            max: MAX_MESSAGE_LENGTH,
        });
    }

    Ok(())
}

/// Validates file size limits
pub fn validate_file_size(size: u64) -> Result<(), ValidationError> {
    if size == 0 {
        return Err(ValidationError::FileSizeZero);
    }

    if size > MAX_FILE_SIZE {
        return Err(ValidationError::FileSizeTooHigh {
            value: size,
            max: MAX_FILE_SIZE,
        });
    }

    Ok(())
}

/// Formats validation errors into a user-friendly message
pub fn format_validation_errors(errors: &[ValidationError]) -> String {
    if errors.is_empty() {
        return "No validation errors".to_string();
    }

    let mut message = String::from("Configuration validation failed:\n");

    for (i, error) in errors.iter().enumerate() {
        message.push_str(&format!("  {}. {}\n", i + 1, error));
    }

    message.push_str("\nPlease correct these issues and try again.");
    message
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_entry(ip: &str) -> IpLimitEntry {
        IpLimitEntry {
            ip: ip.to_string(),
            message_max_length: Some(2048),
            file_max_size: Some(52428800),
        }
    }

    fn create_test_config(entries: Vec<IpLimitEntry>) -> IpLimitsConfig {
        IpLimitsConfig {
            enabled: true,
            whitelist: entries,
        }
    }

    #[test]
    fn test_validate_ipv4_addresses() {
        // Valid IPv4 addresses
        assert!(validate_ip_format("192.168.1.1").is_ok());
        assert!(validate_ip_format("10.0.0.1").is_ok());
        assert!(validate_ip_format("172.16.0.1").is_ok());
        assert!(validate_ip_format("127.0.0.1").is_ok());
        assert!(validate_ip_format("0.0.0.0").is_ok());
        assert!(validate_ip_format("255.255.255.255").is_ok());

        // Invalid IPv4 addresses
        assert!(validate_ip_format("192.168.1.256").is_err());
        assert!(validate_ip_format("192.168.1").is_err());
        assert!(validate_ip_format("192.168.1.1.1").is_err());
        assert!(validate_ip_format("192.168.1.-1").is_err());
        assert!(validate_ip_format("192.168.1.a").is_err());
    }

    #[test]
    fn test_validate_ipv6_addresses() {
        // Valid IPv6 addresses
        assert!(validate_ip_format("2001:db8::1").is_ok());
        assert!(validate_ip_format("::1").is_ok());
        assert!(validate_ip_format("::").is_ok());
        assert!(validate_ip_format("2001:db8:85a3::8a2e:370:7334").is_ok());
        assert!(validate_ip_format("2001:db8:85a3:0:0:8a2e:370:7334").is_ok());

        // Invalid IPv6 addresses
        assert!(validate_ip_format("2001:db8::g").is_err());
        assert!(validate_ip_format("2001:db8:::1").is_err());
        assert!(validate_ip_format("2001:db8:").is_err());
    }

    #[test]
    fn test_validate_cidr_notation() {
        // Valid CIDR notation
        assert!(validate_ip_format("192.168.1.0/24").is_ok());
        assert!(validate_ip_format("10.0.0.0/8").is_ok());
        assert!(validate_ip_format("172.16.0.0/12").is_ok());
        assert!(validate_ip_format("2001:db8::/32").is_ok());
        assert!(validate_ip_format("::1/128").is_ok());

        // Invalid CIDR notation
        assert!(validate_ip_format("192.168.1.0/33").is_err());
        assert!(validate_ip_format("192.168.1.0/-1").is_err());
        assert!(validate_ip_format("192.168.1.0/").is_err());
        assert!(validate_ip_format("192.168.1.0/24/8").is_err());
        assert!(validate_ip_format("not.an.ip/24").is_err());
    }

    #[test]
    fn test_validate_edge_cases() {
        // Empty and whitespace
        assert!(validate_ip_format("").is_err());
        assert!(validate_ip_format("   ").is_err());
        assert!(validate_ip_format("\t").is_err());
        assert!(validate_ip_format("\n").is_err());

        // Invalid characters
        assert!(validate_ip_format("192.168.1.1 extra").is_err());
        assert!(validate_ip_format("@#$%").is_err());
        assert!(validate_ip_format("192.168.1.1@").is_err());
    }

    #[test]
    fn test_validate_message_length_limits() {
        // Valid lengths
        assert!(validate_message_length(1).is_ok());
        assert!(validate_message_length(1024).is_ok());
        assert!(validate_message_length(32768).is_ok());
        assert!(validate_message_length(MAX_MESSAGE_LENGTH).is_ok());

        // Invalid lengths
        assert!(validate_message_length(0).is_err());
        // Can't test MAX_MESSAGE_LENGTH + 1 since it's u16::MAX
    }

    #[test]
    fn test_validate_file_size_limits() {
        // Valid sizes
        assert!(validate_file_size(1).is_ok());
        assert!(validate_file_size(1024).is_ok());
        assert!(validate_file_size(1048576).is_ok());
        assert!(validate_file_size(MAX_FILE_SIZE).is_ok());

        // Invalid sizes
        assert!(validate_file_size(0).is_err());
        assert!(validate_file_size(MAX_FILE_SIZE + 1).is_err());
    }

    #[test]
    fn test_validate_ip_entry() {
        // Valid entry
        let entry = create_test_entry("192.168.1.1");
        assert!(validate_ip_entry(&entry).is_ok());

        // Invalid IP
        let entry = IpLimitEntry {
            ip: "invalid.ip".to_string(),
            message_max_length: Some(2048),
            file_max_size: Some(52428800),
        };
        assert!(validate_ip_entry(&entry).is_err());

        // Invalid message length
        let entry = IpLimitEntry {
            ip: "192.168.1.1".to_string(),
            message_max_length: Some(0),
            file_max_size: Some(52428800),
        };
        assert!(validate_ip_entry(&entry).is_err());

        // Invalid file size
        let entry = IpLimitEntry {
            ip: "192.168.1.1".to_string(),
            message_max_length: Some(2048),
            file_max_size: Some(0),
        };
        assert!(validate_ip_entry(&entry).is_err());
    }

    #[test]
    fn test_validate_ip_limits_config() {
        // Valid config
        let config = create_test_config(vec![
            create_test_entry("192.168.1.1"),
            create_test_entry("10.0.0.0/8"),
        ]);
        assert!(validate_ip_limits_config(&config).is_ok());

        // Empty whitelist with enabled=true (should be OK)
        let config = IpLimitsConfig {
            enabled: true,
            whitelist: vec![],
        };
        assert!(validate_ip_limits_config(&config).is_ok());

        // Duplicate IPs
        let config = create_test_config(vec![
            create_test_entry("192.168.1.1"),
            create_test_entry("192.168.1.1"),
        ]);
        assert!(validate_ip_limits_config(&config).is_err());

        // Invalid entries
        let config = create_test_config(vec![IpLimitEntry {
            ip: "invalid".to_string(),
            message_max_length: Some(0),
            file_max_size: Some(0),
        }]);
        let result = validate_ip_limits_config(&config);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.len() >= 3); // IP format, message length, file size errors
    }

    #[test]
    fn test_format_validation_errors() {
        let errors = vec![
            ValidationError::InvalidIpFormat {
                ip: "invalid".to_string(),
                reason: "test reason".to_string(),
            },
            ValidationError::MessageLengthZero,
        ];

        let formatted = format_validation_errors(&errors);
        assert!(formatted.contains("Configuration validation failed"));
        assert!(formatted.contains("Invalid IP address format"));
        assert!(formatted.contains("Message max length cannot be zero"));
        assert!(formatted.contains("Please correct these issues"));
    }

    #[test]
    fn test_format_no_errors() {
        let errors = vec![];
        let formatted = format_validation_errors(&errors);
        assert_eq!(formatted, "No validation errors");
    }

    #[test]
    fn test_validation_error_messages() {
        // Test specific error message formatting
        let error = ValidationError::InvalidIpFormat {
            ip: "192.168.1.256".to_string(),
            reason: "IPv4 address octets must be between 0 and 255".to_string(),
        };
        assert!(error.to_string().contains("192.168.1.256"));
        assert!(
            error
                .to_string()
                .contains("octets must be between 0 and 255")
        );

        let error = ValidationError::MessageLengthTooHigh {
            value: 60000,
            max: 32000,
        };
        assert!(error.to_string().contains("60000"));
        assert!(error.to_string().contains("32000"));
    }

    #[test]
    fn test_trimming_whitespace() {
        // IPs with leading/trailing whitespace should be handled
        assert!(validate_ip_format("  192.168.1.1  ").is_ok());
        assert!(validate_ip_format("\t10.0.0.0/8\n").is_ok());
        assert!(validate_ip_format(" 2001:db8::1 ").is_ok());
    }

    #[test]
    fn test_boundary_conditions() {
        // Test exact boundary values
        assert!(validate_message_length(MIN_MESSAGE_LENGTH).is_ok());
        assert!(validate_message_length(MAX_MESSAGE_LENGTH).is_ok());
        assert!(validate_file_size(MIN_FILE_SIZE).is_ok());
        assert!(validate_file_size(MAX_FILE_SIZE).is_ok());

        // Test just outside boundaries
        if MIN_MESSAGE_LENGTH > 0 {
            assert!(validate_message_length(MIN_MESSAGE_LENGTH - 1).is_err());
        }
        // Can't test MAX_MESSAGE_LENGTH + 1 since it's u16::MAX
        if MIN_FILE_SIZE > 0 {
            assert!(validate_file_size(MIN_FILE_SIZE - 1).is_err());
        }
        if MAX_FILE_SIZE < u64::MAX {
            assert!(validate_file_size(MAX_FILE_SIZE + 1).is_err());
        }
    }

    #[test]
    fn test_multiple_validation_errors() {
        let entry = IpLimitEntry {
            ip: "invalid.ip.address".to_string(),
            message_max_length: Some(0),
            file_max_size: Some(u64::MAX),
        };

        let result = validate_ip_entry(&entry);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.len() >= 3); // Should have multiple errors
    }

    #[test]
    fn test_ipv6_specific_validations() {
        // IPv6 addresses with various formats
        let valid_ipv6_cases = vec![
            "2001:0db8:85a3:0000:0000:8a2e:0370:7334",
            "2001:db8:85a3:0:0:8a2e:370:7334",
            "2001:db8:85a3::8a2e:370:7334",
            "::1",
            "::",
            "fe80::1",
            "::ffff:192.168.1.1", // IPv4-mapped IPv6
        ];

        for ip in valid_ipv6_cases {
            assert!(
                validate_ip_format(ip).is_ok(),
                "Failed for valid IPv6: {}",
                ip
            );
        }

        let invalid_ipv6_cases = vec![
            "2001:0db8:85a3::8a2e::7334",                   // double ::
            "2001:db8:85a3:0000:0000:8a2e:0370:7334:extra", // too many segments
            "gggg::",                                       // invalid hex
            "2001:db8:::",                                  // too many colons
        ];

        for ip in invalid_ipv6_cases {
            assert!(
                validate_ip_format(ip).is_err(),
                "Should fail for invalid IPv6: {}",
                ip
            );
        }
    }
}
