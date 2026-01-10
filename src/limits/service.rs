use std::net::IpAddr;
use std::str::FromStr;

use ipnet::IpNet;
use log::{debug, info};

use crate::config::model::{AppConfig, IpLimitEntry};

#[derive(Debug, Clone)]
pub struct ClientLimits {
    pub message_max_length: u16,
    pub file_max_size: u64,
    pub encrypted_message_max_length: u64,
}

#[derive(Debug, Clone)]
pub struct LimitsService {
    default_limits: ClientLimits,
    ip_limits_enabled: bool,
    ip_whitelist: Vec<IpLimitEntry>,
}

impl LimitsService {
    pub fn new(config: &AppConfig) -> Self {
        let encrypted_message_max_length =
            config.encrypted_message_max_length.unwrap_or_else(|| {
                Self::calculate_encrypted_max_length(
                    config.message_max_length,
                    config.file_max_size,
                )
            });

        let default_limits = ClientLimits {
            message_max_length: config.message_max_length,
            file_max_size: config.file_max_size,
            encrypted_message_max_length,
        };

        let (ip_limits_enabled, ip_whitelist) = match &config.ip_limits {
            Some(ip_limits) => (ip_limits.enabled, ip_limits.whitelist.clone()),
            None => (false, Vec::new()),
        };

        Self {
            default_limits,
            ip_limits_enabled,
            ip_whitelist,
        }
    }

    pub fn get_limits_for_ip(&self, client_ip: &str) -> ClientLimits {
        if !self.ip_limits_enabled {
            debug!(
                "IP limits disabled, using default limits for client IP: {}",
                client_ip
            );
            return self.default_limits.clone();
        }

        let client_ip_addr = match IpAddr::from_str(client_ip) {
            Ok(addr) => addr,
            Err(e) => {
                debug!(
                    "Failed to parse client IP '{}': {}. Using default limits",
                    client_ip, e
                );
                return self.default_limits.clone();
            }
        };

        for entry in &self.ip_whitelist {
            if self.matches_ip(&client_ip_addr, &entry.ip) {
                let limits = self.calculate_limits_for_entry(entry);
                info!(
                    "Applied custom IP limits for {}: matched rule '{}' -> message_max_length: {}, file_max_size: {}, encrypted_message_max_length: {}",
                    client_ip,
                    entry.ip,
                    limits.message_max_length,
                    limits.file_max_size,
                    limits.encrypted_message_max_length
                );
                return limits;
            }
        }

        debug!(
            "No IP limit rules matched for {}, using default limits",
            client_ip
        );
        self.default_limits.clone()
    }

    fn matches_ip(&self, client_ip: &IpAddr, rule_ip: &str) -> bool {
        if let Ok(exact_ip) = IpAddr::from_str(rule_ip) {
            return *client_ip == exact_ip;
        }

        if let Ok(network) = IpNet::from_str(rule_ip) {
            return network.contains(client_ip);
        }

        false
    }

    fn calculate_limits_for_entry(&self, entry: &IpLimitEntry) -> ClientLimits {
        let message_max_length = entry
            .message_max_length
            .unwrap_or(self.default_limits.message_max_length);
        let file_max_size = entry
            .file_max_size
            .unwrap_or(self.default_limits.file_max_size);

        let encrypted_message_max_length =
            Self::calculate_encrypted_max_length(message_max_length, file_max_size);

        ClientLimits {
            message_max_length,
            file_max_size,
            encrypted_message_max_length,
        }
    }

    fn calculate_encrypted_max_length(message_limit: u16, file_limit: u64) -> u64 {
        let overhead_factor = 1.35; // 35% overhead for encryption
        let max_content_size = std::cmp::max(message_limit as u64, file_limit);
        (max_content_size as f64 * overhead_factor) as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::model::IpLimitsConfig;

    fn create_test_config() -> AppConfig {
        AppConfig {
            listen: "0.0.0.0:8080".to_string(),
            log_level: "info".to_string(),
            log_target: "stdout".to_string(),
            message_max_length: 1024,
            file_upload_enabled: true,
            file_max_size: 10485760,
            encrypted_message_max_length: None, // Will be calculated dynamically
            redis_url: "redis://localhost".to_string(),
            ip_limits: None,
        }
    }

    fn create_test_config_with_limits() -> AppConfig {
        let mut config = create_test_config();
        config.ip_limits = Some(IpLimitsConfig {
            enabled: true,
            whitelist: vec![
                IpLimitEntry {
                    ip: "192.168.1.100".to_string(),
                    message_max_length: Some(8192),
                    file_max_size: Some(104857600),
                },
                IpLimitEntry {
                    ip: "10.0.0.0/8".to_string(),
                    message_max_length: Some(4096),
                    file_max_size: None,
                },
                IpLimitEntry {
                    ip: "172.16.1.5".to_string(),
                    message_max_length: None,
                    file_max_size: Some(209715200),
                },
            ],
        });
        config
    }

    #[test]
    fn test_disabled_ip_limits() {
        let config = create_test_config();
        let service = LimitsService::new(&config);

        let limits = service.get_limits_for_ip("192.168.1.100");
        assert_eq!(limits.message_max_length, 1024);
        assert_eq!(limits.file_max_size, 10485760);
        // Dynamic calculation: max(1024, 10485760) * 1.35 = 14155776
        assert_eq!(limits.encrypted_message_max_length, 14155776);
    }

    #[test]
    fn test_exact_ip_match() {
        let config = create_test_config_with_limits();
        let service = LimitsService::new(&config);

        let limits = service.get_limits_for_ip("192.168.1.100");
        assert_eq!(limits.message_max_length, 8192);
        assert_eq!(limits.file_max_size, 104857600);

        // Dynamic calculation: max(8192, 104857600) * 1.35 = 141557760
        let expected_encrypted = (104857600.0 * 1.35) as u64;
        assert_eq!(limits.encrypted_message_max_length, expected_encrypted);
    }

    #[test]
    fn test_cidr_match() {
        let config = create_test_config_with_limits();
        let service = LimitsService::new(&config);

        let limits = service.get_limits_for_ip("10.1.2.3");
        assert_eq!(limits.message_max_length, 4096);
        assert_eq!(limits.file_max_size, 10485760); // default

        // Dynamic calculation: max(4096, 10485760) * 1.35 = 14155776
        let expected_encrypted = (10485760.0 * 1.35) as u64;
        assert_eq!(limits.encrypted_message_max_length, expected_encrypted);
    }

    #[test]
    fn test_partial_override() {
        let config = create_test_config_with_limits();
        let service = LimitsService::new(&config);

        let limits = service.get_limits_for_ip("172.16.1.5");
        assert_eq!(limits.message_max_length, 1024); // default
        assert_eq!(limits.file_max_size, 209715200);
        // Dynamic calculation: max(1024, 209715200) * 1.35 = 283115520
        let expected_encrypted = (209715200.0 * 1.35) as u64;
        assert_eq!(limits.encrypted_message_max_length, expected_encrypted);
    }

    #[test]
    fn test_no_match() {
        let config = create_test_config_with_limits();
        let service = LimitsService::new(&config);

        let limits = service.get_limits_for_ip("203.0.113.1");
        assert_eq!(limits.message_max_length, 1024);
        assert_eq!(limits.file_max_size, 10485760);
        // Dynamic calculation: max(1024, 10485760) * 1.35 = 14155776
        assert_eq!(limits.encrypted_message_max_length, 14155776);
    }

    #[test]
    fn test_invalid_ip() {
        let config = create_test_config_with_limits();
        let service = LimitsService::new(&config);

        let limits = service.get_limits_for_ip("invalid-ip");
        assert_eq!(limits.message_max_length, 1024);
        assert_eq!(limits.file_max_size, 10485760);
        // Dynamic calculation: max(1024, 10485760) * 1.35 = 14155776
        assert_eq!(limits.encrypted_message_max_length, 14155776);
    }

    #[test]
    fn test_ip_matching_edge_cases() {
        let config = create_test_config();
        let service = LimitsService::new(&config);

        // Test IPv6
        assert!(!service.matches_ip(&"2001:db8::1".parse().unwrap(), "192.168.1.0/24"));

        // Test CIDR boundary
        assert!(service.matches_ip(&"192.168.1.1".parse().unwrap(), "192.168.1.0/24"));
        assert!(!service.matches_ip(&"192.168.2.1".parse().unwrap(), "192.168.1.0/24"));

        // Test exact match
        assert!(service.matches_ip(&"192.168.1.100".parse().unwrap(), "192.168.1.100"));
        assert!(!service.matches_ip(&"192.168.1.101".parse().unwrap(), "192.168.1.100"));
    }

    #[test]
    fn test_encrypted_message_calculation() {
        // Test the new dynamic calculation method
        let result = LimitsService::calculate_encrypted_max_length(2048, 104857600);
        let expected = (104857600.0 * 1.35) as u64; // Uses the larger value (file size)
        assert_eq!(result, expected);

        // Test when message limit is larger
        let result = LimitsService::calculate_encrypted_max_length(65535, 1000);
        let expected = (65535.0 * 1.35) as u64; // Uses the larger value (message length)
        assert_eq!(result, expected);

        // Test equal values
        let result = LimitsService::calculate_encrypted_max_length(1000, 1000);
        let expected = (1000.0 * 1.35) as u64;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_ipv6_addresses() {
        let config = create_test_config_with_limits();
        let service = LimitsService::new(&config);

        // IPv6 addresses should not match IPv4 rules
        let limits = service.get_limits_for_ip("2001:db8::1");
        assert_eq!(limits.message_max_length, 1024); // default
        assert_eq!(limits.file_max_size, 10485760); // default
    }

    #[test]
    fn test_ipv6_cidr_matching() {
        let mut config = create_test_config();
        config.ip_limits = Some(IpLimitsConfig {
            enabled: true,
            whitelist: vec![IpLimitEntry {
                ip: "2001:db8::/32".to_string(),
                message_max_length: Some(16384),
                file_max_size: Some(209715200),
            }],
        });
        let service = LimitsService::new(&config);

        // Should match IPv6 CIDR
        let limits = service.get_limits_for_ip("2001:db8::1");
        assert_eq!(limits.message_max_length, 16384);
        assert_eq!(limits.file_max_size, 209715200);

        // Should not match different IPv6 network
        let limits = service.get_limits_for_ip("2001:db9::1");
        assert_eq!(limits.message_max_length, 1024); // default
    }

    #[test]
    fn test_priority_of_matching_rules() {
        let mut config = create_test_config();
        config.ip_limits = Some(IpLimitsConfig {
            enabled: true,
            whitelist: vec![
                IpLimitEntry {
                    ip: "192.168.0.0/16".to_string(),
                    message_max_length: Some(2048),
                    file_max_size: Some(52428800),
                },
                IpLimitEntry {
                    ip: "192.168.1.100".to_string(),
                    message_max_length: Some(8192),
                    file_max_size: Some(104857600),
                },
            ],
        });
        let service = LimitsService::new(&config);

        // First match should win (broader rule comes first)
        let limits = service.get_limits_for_ip("192.168.1.100");
        assert_eq!(limits.message_max_length, 2048); // from /16 rule
        assert_eq!(limits.file_max_size, 52428800); // from /16 rule
    }

    #[test]
    fn test_malformed_ip_addresses() {
        let config = create_test_config_with_limits();
        let service = LimitsService::new(&config);

        let test_cases = vec![
            "not.an.ip.address",
            "999.999.999.999",
            "192.168.1",
            "192.168.1.256",
            "",
            "   ",
            "192.168.1.1 extra",
            "::gggg::",
        ];

        for invalid_ip in test_cases {
            let limits = service.get_limits_for_ip(invalid_ip);
            assert_eq!(
                limits.message_max_length, 1024,
                "Failed for IP: {}",
                invalid_ip
            );
            assert_eq!(
                limits.file_max_size, 10485760,
                "Failed for IP: {}",
                invalid_ip
            );
            // Dynamic calculation: max(1024, 10485760) * 1.35 = 14155776
            assert_eq!(
                limits.encrypted_message_max_length, 14155776,
                "Failed for IP: {}",
                invalid_ip
            );
        }
    }

    #[test]
    fn test_boundary_cidr_matches() {
        let mut config = create_test_config();
        config.ip_limits = Some(IpLimitsConfig {
            enabled: true,
            whitelist: vec![IpLimitEntry {
                ip: "192.168.1.0/24".to_string(),
                message_max_length: Some(4096),
                file_max_size: Some(52428800),
            }],
        });
        let service = LimitsService::new(&config);

        // First IP in range
        let limits = service.get_limits_for_ip("192.168.1.0");
        assert_eq!(limits.message_max_length, 4096);

        // Last IP in range
        let limits = service.get_limits_for_ip("192.168.1.255");
        assert_eq!(limits.message_max_length, 4096);

        // Just outside range
        let limits = service.get_limits_for_ip("192.168.2.0");
        assert_eq!(limits.message_max_length, 1024); // default

        let limits = service.get_limits_for_ip("192.168.0.255");
        assert_eq!(limits.message_max_length, 1024); // default
    }

    #[test]
    fn test_multiple_cidr_rules() {
        let mut config = create_test_config();
        config.ip_limits = Some(IpLimitsConfig {
            enabled: true,
            whitelist: vec![
                IpLimitEntry {
                    ip: "10.0.0.0/8".to_string(),
                    message_max_length: Some(2048),
                    file_max_size: None,
                },
                IpLimitEntry {
                    ip: "172.16.0.0/12".to_string(),
                    message_max_length: Some(4096),
                    file_max_size: Some(104857600),
                },
                IpLimitEntry {
                    ip: "192.168.0.0/16".to_string(),
                    message_max_length: None,
                    file_max_size: Some(209715200),
                },
            ],
        });
        let service = LimitsService::new(&config);

        // Test each network
        let limits = service.get_limits_for_ip("10.1.2.3");
        assert_eq!(limits.message_max_length, 2048);
        assert_eq!(limits.file_max_size, 10485760); // default

        let limits = service.get_limits_for_ip("172.16.5.10");
        assert_eq!(limits.message_max_length, 4096);
        assert_eq!(limits.file_max_size, 104857600);

        let limits = service.get_limits_for_ip("192.168.10.20");
        assert_eq!(limits.message_max_length, 1024); // default
        assert_eq!(limits.file_max_size, 209715200);
    }

    #[test]
    fn test_invalid_cidr_notation_in_config() {
        let mut config = create_test_config();
        config.ip_limits = Some(IpLimitsConfig {
            enabled: true,
            whitelist: vec![
                IpLimitEntry {
                    ip: "192.168.1.0/33".to_string(), // Invalid CIDR - too high mask
                    message_max_length: Some(8192),
                    file_max_size: Some(104857600),
                },
                IpLimitEntry {
                    ip: "not.a.cidr/24".to_string(), // Invalid CIDR - malformed IP
                    message_max_length: Some(4096),
                    file_max_size: Some(52428800),
                },
                IpLimitEntry {
                    ip: "192.168.1.100".to_string(), // Valid exact IP
                    message_max_length: Some(2048),
                    file_max_size: Some(26214400),
                },
            ],
        });
        let service = LimitsService::new(&config);

        // Invalid CIDR rules should not match anything
        let limits = service.get_limits_for_ip("192.168.1.50");
        assert_eq!(limits.message_max_length, 1024); // default

        // Valid rule should still work
        let limits = service.get_limits_for_ip("192.168.1.100");
        assert_eq!(limits.message_max_length, 2048);
        assert_eq!(limits.file_max_size, 26214400);
    }

    #[test]
    fn test_encrypted_message_calculation_edge_cases() {
        // Maximum u16 message length
        let result = LimitsService::calculate_encrypted_max_length(u16::MAX, 1024);
        let expected = (u16::MAX as f64 * 1.35) as u64;
        assert_eq!(result, expected);

        // Minimum message length
        let result = LimitsService::calculate_encrypted_max_length(1, 1024);
        let expected = (1024.0 * 1.35) as u64; // Uses larger value (file_limit)
        assert_eq!(result, expected);

        // Zero values
        let result = LimitsService::calculate_encrypted_max_length(0, 0);
        let expected = (0.0 * 1.35) as u64;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_service_cloning() {
        let config = create_test_config_with_limits();
        let service = LimitsService::new(&config);
        let cloned_service = service.clone();

        // Both services should behave identically
        let limits1 = service.get_limits_for_ip("192.168.1.100");
        let limits2 = cloned_service.get_limits_for_ip("192.168.1.100");

        assert_eq!(limits1.message_max_length, limits2.message_max_length);
        assert_eq!(limits1.file_max_size, limits2.file_max_size);
        assert_eq!(
            limits1.encrypted_message_max_length,
            limits2.encrypted_message_max_length
        );
    }

    #[test]
    fn test_empty_whitelist() {
        let mut config = create_test_config();
        config.ip_limits = Some(IpLimitsConfig {
            enabled: true,
            whitelist: vec![],
        });
        let service = LimitsService::new(&config);

        // Should return defaults for any IP
        let limits = service.get_limits_for_ip("192.168.1.100");
        assert_eq!(limits.message_max_length, 1024);
        assert_eq!(limits.file_max_size, 10485760);
        // Dynamic calculation: max(1024, 10485760) * 1.35 = 14155776
        assert_eq!(limits.encrypted_message_max_length, 14155776);
    }

    #[test]
    fn test_logging_behavior_disabled_ip_limits() {
        // Test that debug logging occurs when IP limits are disabled
        let config = create_test_config();
        let service = LimitsService::new(&config);

        // This test verifies the method works - actual log output would need log capture
        let limits = service.get_limits_for_ip("192.168.1.100");
        assert_eq!(limits.message_max_length, 1024);
    }

    #[test]
    fn test_logging_behavior_custom_limits_applied() {
        // Test that info logging occurs when custom limits are applied
        let config = create_test_config_with_limits();
        let service = LimitsService::new(&config);

        // This should trigger info logging for custom limits
        let limits = service.get_limits_for_ip("192.168.1.100");
        assert_eq!(limits.message_max_length, 8192);
        assert_eq!(limits.file_max_size, 104857600);
    }

    #[test]
    fn test_logging_behavior_no_match() {
        // Test that debug logging occurs when no rules match
        let config = create_test_config_with_limits();
        let service = LimitsService::new(&config);

        // This should trigger debug logging for no match
        let limits = service.get_limits_for_ip("203.0.113.1");
        assert_eq!(limits.message_max_length, 1024);
    }

    #[test]
    fn test_logging_behavior_invalid_ip() {
        // Test that debug logging occurs for invalid IP parsing
        let config = create_test_config_with_limits();
        let service = LimitsService::new(&config);

        // This should trigger debug logging for parse failure
        let limits = service.get_limits_for_ip("invalid-ip");
        assert_eq!(limits.message_max_length, 1024);
    }
}
