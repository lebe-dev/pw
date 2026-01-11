use axum::{
    extract::{ConnectInfo, Extension, Request},
    http::HeaderMap,
    middleware::Next,
    response::Response,
};
use ipnet::IpNet;
use log::debug;
use std::net::IpAddr;

use crate::config::model::IpLimitsConfig;

pub const CLIENT_IP_EXTENSION_KEY: &str = "client_ip";

#[derive(Debug, Clone)]
pub struct ClientIp(pub IpAddr);

pub struct ClientIpExtractor;

impl ClientIpExtractor {
    pub async fn middleware(
        ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
        ip_limits_config: Option<Extension<Option<IpLimitsConfig>>>,
        mut request: Request,
        next: Next,
    ) -> Response {
        let config = ip_limits_config.as_ref().and_then(|ext| ext.0.as_ref());
        let client_ip = Self::extract_client_ip(request.headers(), addr.ip(), config);
        debug!(
            "Extracted client IP: {} (connection IP: {})",
            client_ip,
            addr.ip()
        );
        request.extensions_mut().insert(ClientIp(client_ip));
        next.run(request).await
    }

    fn extract_client_ip(
        headers: &HeaderMap,
        connection_ip: IpAddr,
        ip_limits_config: Option<&IpLimitsConfig>,
    ) -> IpAddr {
        let should_trust_headers = match ip_limits_config {
            None => {
                debug!("IP limits not configured, trusting proxy headers");
                true
            }
            Some(config) if !config.enabled => {
                debug!("IP limits disabled, trusting proxy headers");
                true
            }
            Some(config) if config.trusted_proxies.is_empty() => {
                debug!(
                    "IP limits enabled but trusted-proxies is empty, ignoring proxy headers for security"
                );
                false
            }
            Some(config) => Self::is_trusted_proxy(&connection_ip, &config.trusted_proxies),
        };

        if !should_trust_headers {
            debug!(
                "connection IP {} is not a trusted proxy, using connection IP",
                connection_ip
            );
            return connection_ip;
        }

        if let Some(forwarded_for) = headers.get("x-forwarded-for")
            && let Ok(header_value) = forwarded_for.to_str()
        {
            debug!("found X-Forwarded-For header: {}", header_value);
            if let Some(first_ip) = Self::parse_forwarded_for(header_value) {
                debug!("using IP from X-Forwarded-For: {}", first_ip);
                return first_ip;
            } else {
                debug!("failed to parse valid IP from X-Forwarded-For header, trying X-Real-IP");
            }
        }

        if let Some(real_ip) = headers.get("x-real-ip")
            && let Ok(header_value) = real_ip.to_str()
        {
            debug!("found X-Real-IP header: {}", header_value);
            if let Ok(ip) = header_value.trim().parse::<IpAddr>() {
                debug!("using IP from X-Real-IP: {}", ip);
                return ip;
            } else {
                debug!("failed to parse IP from X-Real-IP header, using connection IP");
            }
        }

        debug!(
            "no valid proxy headers found, using connection IP: {}",
            connection_ip
        );
        connection_ip
    }

    fn is_trusted_proxy(connection_ip: &IpAddr, trusted_proxies: &[String]) -> bool {
        for trusted in trusted_proxies {
            if Self::matches_ip_or_cidr(connection_ip, trusted) {
                debug!(
                    "connection IP {} matched trusted proxy {}",
                    connection_ip, trusted
                );
                return true;
            }
        }
        debug!(
            "connection IP {} not in trusted proxies list",
            connection_ip
        );
        false
    }

    fn matches_ip_or_cidr(ip: &IpAddr, pattern: &str) -> bool {
        if let Ok(pattern_ip) = pattern.parse::<IpAddr>() {
            return ip == &pattern_ip;
        }

        if let Ok(network) = pattern.parse::<IpNet>() {
            return network.contains(ip);
        }

        false
    }

    fn parse_forwarded_for(header_value: &str) -> Option<IpAddr> {
        header_value.split(',').next().and_then(|ip_str| {
            let trimmed = ip_str.trim();
            if Self::is_valid_ip_format(trimmed) {
                trimmed.parse::<IpAddr>().ok()
            } else {
                None
            }
        })
    }

    fn is_valid_ip_format(ip_str: &str) -> bool {
        if ip_str.is_empty() || ip_str.len() > 45 {
            return false;
        }

        ip_str.parse::<IpAddr>().is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderMap;
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

    #[test]
    fn test_extract_client_ip_from_connection() {
        let headers = HeaderMap::new();
        let connection_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100));

        let result = ClientIpExtractor::extract_client_ip(&headers, connection_ip, None);
        assert_eq!(result, connection_ip);
    }

    #[test]
    fn test_extract_client_ip_from_x_forwarded_for() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "x-forwarded-for",
            "203.0.113.195, 198.51.100.178".parse().unwrap(),
        );
        let connection_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100));

        let result = ClientIpExtractor::extract_client_ip(&headers, connection_ip, None);
        assert_eq!(result, IpAddr::V4(Ipv4Addr::new(203, 0, 113, 195)));
    }

    #[test]
    fn test_extract_client_ip_from_x_real_ip() {
        let mut headers = HeaderMap::new();
        headers.insert("x-real-ip", "203.0.113.195".parse().unwrap());
        let connection_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100));

        let result = ClientIpExtractor::extract_client_ip(&headers, connection_ip, None);
        assert_eq!(result, IpAddr::V4(Ipv4Addr::new(203, 0, 113, 195)));
    }

    #[test]
    fn test_x_forwarded_for_takes_precedence_over_x_real_ip() {
        let mut headers = HeaderMap::new();
        headers.insert("x-forwarded-for", "203.0.113.195".parse().unwrap());
        headers.insert("x-real-ip", "198.51.100.178".parse().unwrap());
        let connection_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100));

        let result = ClientIpExtractor::extract_client_ip(&headers, connection_ip, None);
        assert_eq!(result, IpAddr::V4(Ipv4Addr::new(203, 0, 113, 195)));
    }

    #[test]
    fn test_invalid_x_forwarded_for_falls_back_to_x_real_ip() {
        let mut headers = HeaderMap::new();
        headers.insert("x-forwarded-for", "invalid-ip".parse().unwrap());
        headers.insert("x-real-ip", "203.0.113.195".parse().unwrap());
        let connection_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100));

        let result = ClientIpExtractor::extract_client_ip(&headers, connection_ip, None);
        assert_eq!(result, IpAddr::V4(Ipv4Addr::new(203, 0, 113, 195)));
    }

    #[test]
    fn test_invalid_headers_fall_back_to_connection_ip() {
        let mut headers = HeaderMap::new();
        headers.insert("x-forwarded-for", "invalid-ip".parse().unwrap());
        headers.insert("x-real-ip", "also-invalid".parse().unwrap());
        let connection_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100));

        let result = ClientIpExtractor::extract_client_ip(&headers, connection_ip, None);
        assert_eq!(result, connection_ip);
    }

    #[test]
    fn test_ipv6_addresses() {
        let mut headers = HeaderMap::new();
        headers.insert("x-forwarded-for", "2001:db8::1".parse().unwrap());
        let connection_ip = IpAddr::V6(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 0x2));

        let result = ClientIpExtractor::extract_client_ip(&headers, connection_ip, None);
        assert_eq!(
            result,
            IpAddr::V6(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 0x1))
        );
    }

    #[test]
    fn test_parse_forwarded_for_multiple_ips() {
        let result =
            ClientIpExtractor::parse_forwarded_for("203.0.113.195, 198.51.100.178, 192.168.1.1");
        assert_eq!(result, Some(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 195))));
    }

    #[test]
    fn test_parse_forwarded_for_single_ip() {
        let result = ClientIpExtractor::parse_forwarded_for("203.0.113.195");
        assert_eq!(result, Some(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 195))));
    }

    #[test]
    fn test_parse_forwarded_for_with_whitespace() {
        let result = ClientIpExtractor::parse_forwarded_for("  203.0.113.195  , 198.51.100.178");
        assert_eq!(result, Some(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 195))));
    }

    #[test]
    fn test_parse_forwarded_for_invalid_ip() {
        let result = ClientIpExtractor::parse_forwarded_for("invalid-ip, 198.51.100.178");
        assert_eq!(result, None);
    }

    #[test]
    fn test_is_valid_ip_format() {
        assert!(ClientIpExtractor::is_valid_ip_format("192.168.1.1"));
        assert!(ClientIpExtractor::is_valid_ip_format("2001:db8::1"));
        assert!(ClientIpExtractor::is_valid_ip_format("::1"));
        assert!(ClientIpExtractor::is_valid_ip_format("127.0.0.1"));

        assert!(!ClientIpExtractor::is_valid_ip_format(""));
        assert!(!ClientIpExtractor::is_valid_ip_format("not-an-ip"));
        assert!(!ClientIpExtractor::is_valid_ip_format("999.999.999.999"));
        assert!(!ClientIpExtractor::is_valid_ip_format("192.168.1"));
        assert!(!ClientIpExtractor::is_valid_ip_format(
            "a".repeat(46).as_str()
        ));
    }

    #[test]
    fn test_security_header_validation() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "x-forwarded-for",
            "0.0.0.0<script>alert('xss')</script>".parse().unwrap(),
        );
        let connection_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100));

        let result = ClientIpExtractor::extract_client_ip(&headers, connection_ip, None);
        assert_eq!(result, connection_ip);
    }

    #[test]
    fn test_empty_header_values() {
        let mut headers = HeaderMap::new();
        headers.insert("x-forwarded-for", "".parse().unwrap());
        headers.insert("x-real-ip", "".parse().unwrap());
        let connection_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100));

        let result = ClientIpExtractor::extract_client_ip(&headers, connection_ip, None);
        assert_eq!(result, connection_ip);
    }

    #[test]
    fn test_extremely_long_forwarded_for_header() {
        let mut headers = HeaderMap::new();
        let long_header = format!("{}, 192.168.1.1", "1.1.1.1,".repeat(1000));
        headers.insert("x-forwarded-for", long_header.parse().unwrap());
        let connection_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100));

        let result = ClientIpExtractor::extract_client_ip(&headers, connection_ip, None);
        assert_eq!(result, IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)));
    }

    #[test]
    fn test_forwarded_for_with_ports() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "x-forwarded-for",
            "203.0.113.195:8080, 198.51.100.178:9000".parse().unwrap(),
        );
        let connection_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100));

        // Should fail to parse and fall back to connection IP
        let result = ClientIpExtractor::extract_client_ip(&headers, connection_ip, None);
        assert_eq!(result, connection_ip);
    }

    #[test]
    fn test_case_insensitive_headers() {
        let mut headers = HeaderMap::new();
        headers.insert("X-FORWARDED-FOR", "203.0.113.195".parse().unwrap());
        headers.insert("X-REAL-IP", "198.51.100.178".parse().unwrap());
        let connection_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100));

        // Headers should be case-insensitive (handled by HeaderMap)
        let result = ClientIpExtractor::extract_client_ip(&headers, connection_ip, None);
        assert_eq!(result, IpAddr::V4(Ipv4Addr::new(203, 0, 113, 195)));
    }

    #[test]
    fn test_ipv6_in_headers() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "x-forwarded-for",
            "2001:db8::8a2e:370:0:7334".parse().unwrap(),
        );
        let connection_ip = IpAddr::V6(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 0x1));

        let result = ClientIpExtractor::extract_client_ip(&headers, connection_ip, None);
        assert_eq!(
            result,
            IpAddr::V6(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0x8a2e, 0x370, 0, 0x7334))
        );
    }

    #[test]
    fn test_mixed_ipv4_ipv6_in_forwarded_for() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "x-forwarded-for",
            "203.0.113.195, 2001:db8::1, 198.51.100.178"
                .parse()
                .unwrap(),
        );
        let connection_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100));

        // Should pick the first valid IP
        let result = ClientIpExtractor::extract_client_ip(&headers, connection_ip, None);
        assert_eq!(result, IpAddr::V4(Ipv4Addr::new(203, 0, 113, 195)));
    }

    #[test]
    fn test_x_real_ip_with_ipv6() {
        let mut headers = HeaderMap::new();
        headers.insert("x-real-ip", "2001:db8::1".parse().unwrap());
        let connection_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100));

        let result = ClientIpExtractor::extract_client_ip(&headers, connection_ip, None);
        assert_eq!(
            result,
            IpAddr::V6(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 0x1))
        );
    }

    #[test]
    fn test_forwarded_for_with_brackets_ipv6() {
        let mut headers = HeaderMap::new();
        headers.insert("x-forwarded-for", "[2001:db8::1]:8080".parse().unwrap());
        let connection_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100));

        // Should fail to parse (includes port) and fall back
        let result = ClientIpExtractor::extract_client_ip(&headers, connection_ip, None);
        assert_eq!(result, connection_ip);
    }

    #[test]
    fn test_malicious_injection_attempts() {
        let mut headers = HeaderMap::new();
        let malicious_payloads = vec![
            "192.168.1.1; DROP TABLE users;--",
            "../../../etc/passwd",
            "file:///etc/passwd",
            "192.168.1.1 extra_data",
        ];

        let connection_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100));

        for payload in malicious_payloads {
            headers.clear();
            // Only test payloads that are valid header values
            if let Ok(header_value) = payload.parse() {
                headers.insert("x-forwarded-for", header_value);
                let result = ClientIpExtractor::extract_client_ip(&headers, connection_ip, None);
                assert_eq!(result, connection_ip, "Failed for payload: {}", payload);
            }
        }
    }

    #[test]
    fn test_header_with_control_characters() {
        let mut headers = HeaderMap::new();
        let connection_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100));

        // Insert control character that should be rejected
        if let Ok(header_value) = "192.168.1.1\x01".parse() {
            headers.insert("x-forwarded-for", header_value);
            let result = ClientIpExtractor::extract_client_ip(&headers, connection_ip, None);
            assert_eq!(result, connection_ip);
        }
    }

    #[test]
    fn test_private_ip_addresses_in_headers() {
        let mut headers = HeaderMap::new();
        let test_cases = vec![
            ("10.0.0.1", IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1))),
            ("172.16.0.1", IpAddr::V4(Ipv4Addr::new(172, 16, 0, 1))),
            ("192.168.1.1", IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1))),
            ("127.0.0.1", IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
            ("::1", IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1))),
            (
                "fc00::1",
                IpAddr::V6(Ipv6Addr::new(0xfc00, 0, 0, 0, 0, 0, 0, 1)),
            ),
        ];

        let connection_ip = IpAddr::V4(Ipv4Addr::new(203, 0, 113, 1));

        for (ip_str, expected_ip) in test_cases {
            headers.clear();
            headers.insert("x-forwarded-for", ip_str.parse().unwrap());
            let result = ClientIpExtractor::extract_client_ip(&headers, connection_ip, None);
            assert_eq!(result, expected_ip, "Failed for IP: {}", ip_str);
        }
    }

    #[test]
    fn test_is_valid_ip_format_comprehensive() {
        // Valid IPv4 addresses
        let valid_ipv4 = vec![
            "0.0.0.0",
            "127.0.0.1",
            "192.168.1.1",
            "255.255.255.255",
            "203.0.113.195",
        ];

        for ip in valid_ipv4 {
            assert!(
                ClientIpExtractor::is_valid_ip_format(ip),
                "Should be valid: {}",
                ip
            );
        }

        // Valid IPv6 addresses
        let valid_ipv6 = vec![
            "::1",
            "2001:db8::1",
            "2001:db8:85a3::8a2e:370:7334",
            "::ffff:192.0.2.1",
            "fe80::1%lo0", // This will fail because of the zone identifier
        ];

        for ip in &valid_ipv6[..4] {
            // Skip the zone identifier one
            assert!(
                ClientIpExtractor::is_valid_ip_format(ip),
                "Should be valid: {}",
                ip
            );
        }

        // Invalid formats
        let invalid = vec![
            "",
            " ",
            "not-an-ip",
            "999.999.999.999",
            "192.168.1",
            "192.168.1.1.1",
            "192.168.1.256",
            "::gggg::",
            "2001:db8::1::2",   // Double ::
            "192.168.1.1:8080", // With port
            "[2001:db8::1]",    // With brackets
            "fe80::1%lo0",      // Zone identifier
        ];

        for ip in invalid {
            assert!(
                !ClientIpExtractor::is_valid_ip_format(ip),
                "Should be invalid: {}",
                ip
            );
        }
    }

    #[test]
    fn test_performance_with_many_forwarded_ips() {
        let mut headers = HeaderMap::new();
        let many_ips: Vec<String> = (1..=100).map(|i| format!("192.168.1.{}", i)).collect();
        let forwarded_header = many_ips.join(", ");
        headers.insert("x-forwarded-for", forwarded_header.parse().unwrap());

        let connection_ip = IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1));

        let result = ClientIpExtractor::extract_client_ip(&headers, connection_ip, None);
        assert_eq!(result, IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1))); // First IP
    }

    // Trusted proxy tests
    use crate::config::model::{IpLimitEntry, IpLimitsConfig};

    #[test]
    fn test_trusted_proxy_exact_match() {
        let mut headers = HeaderMap::new();
        headers.insert("x-forwarded-for", "203.0.113.195".parse().unwrap());
        let connection_ip = IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1));

        let config = IpLimitsConfig {
            enabled: true,
            whitelist: vec![],
            trusted_proxies: vec!["10.0.0.1".to_string()],
        };

        let result = ClientIpExtractor::extract_client_ip(&headers, connection_ip, Some(&config));
        // Should trust the proxy and use forwarded IP
        assert_eq!(result, IpAddr::V4(Ipv4Addr::new(203, 0, 113, 195)));
    }

    #[test]
    fn test_trusted_proxy_cidr_match() {
        let mut headers = HeaderMap::new();
        headers.insert("x-forwarded-for", "203.0.113.195".parse().unwrap());
        let connection_ip = IpAddr::V4(Ipv4Addr::new(10, 0, 0, 50));

        let config = IpLimitsConfig {
            enabled: true,
            whitelist: vec![],
            trusted_proxies: vec!["10.0.0.0/8".to_string()],
        };

        let result = ClientIpExtractor::extract_client_ip(&headers, connection_ip, Some(&config));
        // Should trust the proxy (in CIDR range) and use forwarded IP
        assert_eq!(result, IpAddr::V4(Ipv4Addr::new(203, 0, 113, 195)));
    }

    #[test]
    fn test_untrusted_proxy_ignores_headers() {
        let mut headers = HeaderMap::new();
        headers.insert("x-forwarded-for", "203.0.113.195".parse().unwrap());
        let connection_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100));

        let config = IpLimitsConfig {
            enabled: true,
            whitelist: vec![],
            trusted_proxies: vec!["10.0.0.1".to_string()],
        };

        let result = ClientIpExtractor::extract_client_ip(&headers, connection_ip, Some(&config));
        // Should NOT trust proxy (different IP) and use connection IP
        assert_eq!(result, connection_ip);
    }

    #[test]
    fn test_empty_trusted_proxies_secure_default() {
        let mut headers = HeaderMap::new();
        headers.insert("x-forwarded-for", "203.0.113.195".parse().unwrap());
        let connection_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100));

        let config = IpLimitsConfig {
            enabled: true,
            whitelist: vec![],
            trusted_proxies: vec![], // Empty = secure by default
        };

        let result = ClientIpExtractor::extract_client_ip(&headers, connection_ip, Some(&config));
        // Should NOT trust any proxy (secure by default) and use connection IP
        assert_eq!(result, connection_ip);
    }

    #[test]
    fn test_disabled_ip_limits_trusts_headers() {
        let mut headers = HeaderMap::new();
        headers.insert("x-forwarded-for", "203.0.113.195".parse().unwrap());
        let connection_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100));

        let config = IpLimitsConfig {
            enabled: false, // Disabled
            whitelist: vec![],
            trusted_proxies: vec![], // Even though empty, should trust because disabled
        };

        let result = ClientIpExtractor::extract_client_ip(&headers, connection_ip, Some(&config));
        // Should trust headers because IP limits are disabled
        assert_eq!(result, IpAddr::V4(Ipv4Addr::new(203, 0, 113, 195)));
    }

    #[test]
    fn test_no_ip_limits_config_trusts_headers() {
        let mut headers = HeaderMap::new();
        headers.insert("x-forwarded-for", "203.0.113.195".parse().unwrap());
        let connection_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100));

        let result = ClientIpExtractor::extract_client_ip(&headers, connection_ip, None);
        // Should trust headers (backward compatibility)
        assert_eq!(result, IpAddr::V4(Ipv4Addr::new(203, 0, 113, 195)));
    }

    #[test]
    fn test_ipv6_trusted_proxy() {
        let mut headers = HeaderMap::new();
        headers.insert("x-forwarded-for", "203.0.113.195".parse().unwrap());
        let connection_ip = IpAddr::V6(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 0x1));

        let config = IpLimitsConfig {
            enabled: true,
            whitelist: vec![],
            trusted_proxies: vec!["2001:db8::1".to_string()],
        };

        let result = ClientIpExtractor::extract_client_ip(&headers, connection_ip, Some(&config));
        // Should trust IPv6 proxy and use forwarded IP
        assert_eq!(result, IpAddr::V4(Ipv4Addr::new(203, 0, 113, 195)));
    }

    #[test]
    fn test_trusted_proxy_ipv6_cidr() {
        let mut headers = HeaderMap::new();
        headers.insert("x-forwarded-for", "203.0.113.195".parse().unwrap());
        let connection_ip = IpAddr::V6(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 0x100));

        let config = IpLimitsConfig {
            enabled: true,
            whitelist: vec![],
            trusted_proxies: vec!["2001:db8::/32".to_string()],
        };

        let result = ClientIpExtractor::extract_client_ip(&headers, connection_ip, Some(&config));
        // Should trust IPv6 proxy (in CIDR range) and use forwarded IP
        assert_eq!(result, IpAddr::V4(Ipv4Addr::new(203, 0, 113, 195)));
    }

    #[test]
    fn test_trusted_proxy_mismatch() {
        let mut headers = HeaderMap::new();
        headers.insert("x-forwarded-for", "203.0.113.195".parse().unwrap());
        let connection_ip = IpAddr::V4(Ipv4Addr::new(172, 16, 0, 1));

        let config = IpLimitsConfig {
            enabled: true,
            whitelist: vec![],
            trusted_proxies: vec!["10.0.0.0/8".to_string(), "192.168.0.0/16".to_string()],
        };

        let result = ClientIpExtractor::extract_client_ip(&headers, connection_ip, Some(&config));
        // Should NOT trust proxy (not in any trusted range) and use connection IP
        assert_eq!(result, connection_ip);
    }

    #[test]
    fn test_multiple_trusted_proxies() {
        let mut headers = HeaderMap::new();
        headers.insert("x-forwarded-for", "203.0.113.195".parse().unwrap());

        let config = IpLimitsConfig {
            enabled: true,
            whitelist: vec![],
            trusted_proxies: vec![
                "10.0.0.1".to_string(),
                "172.16.0.0/12".to_string(),
                "192.168.1.100".to_string(),
            ],
        };

        // Test first trusted proxy
        let connection_ip = IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1));
        let result = ClientIpExtractor::extract_client_ip(&headers, connection_ip, Some(&config));
        assert_eq!(result, IpAddr::V4(Ipv4Addr::new(203, 0, 113, 195)));

        // Test CIDR range
        let connection_ip = IpAddr::V4(Ipv4Addr::new(172, 20, 0, 50));
        let result = ClientIpExtractor::extract_client_ip(&headers, connection_ip, Some(&config));
        assert_eq!(result, IpAddr::V4(Ipv4Addr::new(203, 0, 113, 195)));

        // Test third trusted proxy
        let connection_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100));
        let result = ClientIpExtractor::extract_client_ip(&headers, connection_ip, Some(&config));
        assert_eq!(result, IpAddr::V4(Ipv4Addr::new(203, 0, 113, 195)));

        // Test untrusted IP
        let connection_ip = IpAddr::V4(Ipv4Addr::new(198, 51, 100, 1));
        let result = ClientIpExtractor::extract_client_ip(&headers, connection_ip, Some(&config));
        assert_eq!(result, connection_ip); // Should use connection IP
    }

    #[test]
    fn test_matches_ip_or_cidr() {
        // Exact IPv4 match
        let ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100));
        assert!(ClientIpExtractor::matches_ip_or_cidr(&ip, "192.168.1.100"));
        assert!(!ClientIpExtractor::matches_ip_or_cidr(&ip, "192.168.1.101"));

        // IPv4 CIDR match
        assert!(ClientIpExtractor::matches_ip_or_cidr(&ip, "192.168.1.0/24"));
        assert!(ClientIpExtractor::matches_ip_or_cidr(&ip, "192.168.0.0/16"));
        assert!(!ClientIpExtractor::matches_ip_or_cidr(&ip, "10.0.0.0/8"));

        // Exact IPv6 match
        let ipv6 = IpAddr::V6(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 0x1));
        assert!(ClientIpExtractor::matches_ip_or_cidr(&ipv6, "2001:db8::1"));
        assert!(!ClientIpExtractor::matches_ip_or_cidr(&ipv6, "2001:db8::2"));

        // IPv6 CIDR match
        assert!(ClientIpExtractor::matches_ip_or_cidr(
            &ipv6,
            "2001:db8::/32"
        ));
        assert!(!ClientIpExtractor::matches_ip_or_cidr(
            &ipv6,
            "2001:db9::/32"
        ));

        // Invalid patterns
        assert!(!ClientIpExtractor::matches_ip_or_cidr(&ip, "invalid"));
        assert!(!ClientIpExtractor::matches_ip_or_cidr(&ip, ""));
    }

    #[test]
    fn test_trusted_proxy_with_x_real_ip() {
        let mut headers = HeaderMap::new();
        headers.insert("x-real-ip", "203.0.113.195".parse().unwrap());
        let connection_ip = IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1));

        let config = IpLimitsConfig {
            enabled: true,
            whitelist: vec![],
            trusted_proxies: vec!["10.0.0.1".to_string()],
        };

        let result = ClientIpExtractor::extract_client_ip(&headers, connection_ip, Some(&config));
        // Should trust proxy and use X-Real-IP
        assert_eq!(result, IpAddr::V4(Ipv4Addr::new(203, 0, 113, 195)));
    }

    #[test]
    fn test_untrusted_proxy_ignores_x_real_ip() {
        let mut headers = HeaderMap::new();
        headers.insert("x-real-ip", "203.0.113.195".parse().unwrap());
        let connection_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100));

        let config = IpLimitsConfig {
            enabled: true,
            whitelist: vec![],
            trusted_proxies: vec!["10.0.0.1".to_string()],
        };

        let result = ClientIpExtractor::extract_client_ip(&headers, connection_ip, Some(&config));
        // Should NOT trust proxy and use connection IP
        assert_eq!(result, connection_ip);
    }

    #[test]
    fn test_spoofing_attack_prevention() {
        // Simulate attacker trying to spoof IP with X-Forwarded-For
        let mut headers = HeaderMap::new();
        headers.insert("x-forwarded-for", "192.168.1.100".parse().unwrap()); // Whitelisted IP
        let attacker_ip = IpAddr::V4(Ipv4Addr::new(203, 0, 113, 50)); // Attacker's real IP

        let config = IpLimitsConfig {
            enabled: true,
            whitelist: vec![IpLimitEntry {
                ip: "192.168.1.100".to_string(),
                message_max_length: Some(8192),
                file_max_size: Some(104857600),
            }],
            trusted_proxies: vec!["10.0.0.1".to_string()], // Only trust this proxy
        };

        let result = ClientIpExtractor::extract_client_ip(&headers, attacker_ip, Some(&config));
        // Attack prevented: should use attacker's real IP, not spoofed IP
        assert_eq!(result, attacker_ip);
        assert_ne!(result, IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)));
    }
}
