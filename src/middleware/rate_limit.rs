use axum::{Extension, extract::Request, middleware::Next, response::Response};
use ipnet::IpNet;
use log::debug;
use std::net::IpAddr;
use tower_governor::{GovernorLayer, governor::GovernorConfigBuilder, key_extractor::KeyExtractor};

use crate::config::model::{IpLimitsConfig, RateLimitConfig};
use crate::middleware::client_ip::ClientIp;

/// Middleware function that checks whitelist before rate limiting
pub async fn rate_limit_middleware(
    Extension(ip_limits): Extension<Option<IpLimitsConfig>>,
    mut request: Request,
    next: Next,
) -> Response {
    let client_ip = request.extensions().get::<ClientIp>().map(|ip| ip.0);

    if let Some(ip) = client_ip {
        if let Some(ref limits_config) = ip_limits {
            if limits_config.enabled && is_whitelisted(&ip, limits_config) {
                debug!("IP {} is whitelisted, bypassing rate limit", ip);
                request.extensions_mut().insert(BypassRateLimit);
            }
        }
    }

    next.run(request).await
}

/// Marker type to indicate rate limiting should be bypassed
#[derive(Clone, Copy)]
struct BypassRateLimit;

/// Check if IP is in whitelist
fn is_whitelisted(ip: &IpAddr, config: &IpLimitsConfig) -> bool {
    for entry in &config.whitelist {
        if matches_ip_or_cidr(ip, &entry.ip) {
            return true;
        }
    }
    false
}

/// Match IP against pattern (exact IP or CIDR)
fn matches_ip_or_cidr(ip: &IpAddr, pattern: &str) -> bool {
    if let Ok(pattern_ip) = pattern.parse::<IpAddr>() {
        return ip == &pattern_ip;
    }

    if let Ok(network) = pattern.parse::<IpNet>() {
        return network.contains(ip);
    }

    false
}

/// Custom key extractor that uses ClientIp from extension
/// Returns None if request should bypass rate limiting
#[derive(Clone)]
pub struct IpKeyExtractor;

impl KeyExtractor for IpKeyExtractor {
    type Key = IpAddr;

    fn extract<B>(&self, req: &Request<B>) -> Result<Self::Key, tower_governor::GovernorError> {
        if req.extensions().get::<BypassRateLimit>().is_some() {
            debug!("request has bypass marker, allowing through");
            return Err(tower_governor::GovernorError::UnableToExtractKey);
        }

        req.extensions()
            .get::<ClientIp>()
            .map(|ip| ip.0)
            .ok_or(tower_governor::GovernorError::UnableToExtractKey)
    }
}

/// Build governor configuration from app config
pub fn create_rate_limit_layer(
    config: &RateLimitConfig,
) -> GovernorLayer<
    IpKeyExtractor,
    governor::middleware::NoOpMiddleware<governor::clock::QuantaInstant>,
    axum::body::Body,
> {
    // Convert requests per minute to requests per second
    // Governor uses period per request, so for 60 req/min = 1 req/sec, period = 1 second
    let requests_per_second = config.requests_per_minute / 60;
    let requests_per_second = requests_per_second.max(1); // Minimum 1 req/sec

    let governor_config = GovernorConfigBuilder::default()
        .per_second(requests_per_second as u64)
        .burst_size(config.burst_size)
        .key_extractor(IpKeyExtractor)
        .finish()
        .expect("Failed to build rate limit config");

    GovernorLayer::new(governor_config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{Ipv4Addr, Ipv6Addr};

    #[test]
    fn test_matches_ip_or_cidr_exact_ipv4() {
        let ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100));
        assert!(matches_ip_or_cidr(&ip, "192.168.1.100"));
        assert!(!matches_ip_or_cidr(&ip, "192.168.1.101"));
    }

    #[test]
    fn test_matches_ip_or_cidr_ipv4_cidr() {
        let ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100));
        assert!(matches_ip_or_cidr(&ip, "192.168.1.0/24"));
        assert!(matches_ip_or_cidr(&ip, "192.168.0.0/16"));
        assert!(!matches_ip_or_cidr(&ip, "10.0.0.0/8"));
    }

    #[test]
    fn test_matches_ip_or_cidr_exact_ipv6() {
        let ip = IpAddr::V6(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 0x1));
        assert!(matches_ip_or_cidr(&ip, "2001:db8::1"));
        assert!(!matches_ip_or_cidr(&ip, "2001:db8::2"));
    }

    #[test]
    fn test_matches_ip_or_cidr_ipv6_cidr() {
        let ip = IpAddr::V6(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 0x1));
        assert!(matches_ip_or_cidr(&ip, "2001:db8::/32"));
        assert!(!matches_ip_or_cidr(&ip, "2001:db9::/32"));
    }

    #[test]
    fn test_matches_ip_or_cidr_invalid_pattern() {
        let ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100));
        assert!(!matches_ip_or_cidr(&ip, "invalid"));
        assert!(!matches_ip_or_cidr(&ip, ""));
    }
}
