use axum::{Extension, extract::Request, middleware::Next, response::Response};
use ipnet::IpNet;
use log::{debug, info, warn};
use std::net::IpAddr;
use std::time::Duration;
use tower_governor::{GovernorLayer, governor::GovernorConfigBuilder, key_extractor::KeyExtractor};

use crate::config::model::{IpLimitsConfig, RouteRateLimitConfig};
use crate::middleware::client_ip::ClientIp;

/// Middleware function that checks whitelist before rate limiting
pub async fn rate_limit_middleware(
    Extension(ip_limits): Extension<Option<IpLimitsConfig>>,
    mut request: Request,
    next: Next,
) -> Response {
    let client_ip = request.extensions().get::<ClientIp>().map(|ip| ip.0);

    if let Some(ip) = client_ip {
        debug!("processing rate limit check for IP: {}", ip);

        if let Some(ref limits_config) = ip_limits {
            if !limits_config.enabled {
                debug!("IP limits are disabled in config");
            } else if is_whitelisted(&ip, limits_config) {
                debug!("IP {} is whitelisted, bypassing rate limit", ip);
                request.extensions_mut().insert(BypassRateLimit);
            } else {
                debug!("IP {} will be subject to rate limiting", ip);
            }
        } else {
            debug!("no IP limits configuration found");
        }
    } else {
        warn!("unable to extract client IP for rate limiting");
    }

    next.run(request).await
}

/// Marker type to indicate rate limiting should be bypassed
#[derive(Clone, Copy)]
struct BypassRateLimit;

/// Check if IP is in whitelist
fn is_whitelisted(ip: &IpAddr, config: &IpLimitsConfig) -> bool {
    debug!(
        "checking if IP {} is in whitelist ({} entries)",
        ip,
        config.whitelist.len()
    );

    for entry in &config.whitelist {
        if matches_ip_or_cidr(ip, &entry.ip) {
            debug!("IP {} matched whitelist entry: {}", ip, entry.ip);
            return true;
        }
    }

    debug!("IP {} not found in whitelist", ip);
    false
}

/// Match IP against pattern (exact IP or CIDR)
fn matches_ip_or_cidr(ip: &IpAddr, pattern: &str) -> bool {
    if let Ok(pattern_ip) = pattern.parse::<IpAddr>() {
        let matches = ip == &pattern_ip;
        debug!("exact IP match check: {} vs {} = {}", ip, pattern, matches);
        return matches;
    }

    if let Ok(network) = pattern.parse::<IpNet>() {
        let contains = network.contains(ip);
        debug!("CIDR match check: {} in {} = {}", ip, pattern, contains);
        return contains;
    }

    warn!("invalid IP/CIDR pattern in whitelist: {}", pattern);
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
            debug!("request has bypass marker, allowing through without rate limiting");
            return Err(tower_governor::GovernorError::UnableToExtractKey);
        }

        match req.extensions().get::<ClientIp>() {
            Some(client_ip) => {
                debug!("extracting IP {} for rate limiting", client_ip.0);
                Ok(client_ip.0)
            }
            None => {
                warn!("failed to extract client IP for rate limiting");
                Err(tower_governor::GovernorError::UnableToExtractKey)
            }
        }
    }
}

/// Build governor configuration from route config
pub fn create_rate_limit_layer(
    config: &RouteRateLimitConfig,
) -> GovernorLayer<
    IpKeyExtractor,
    governor::middleware::NoOpMiddleware<governor::clock::QuantaInstant>,
    axum::body::Body,
> {
    // Calculate period between requests: 60 seconds / requests_per_minute
    // Example: 1 req/min → 60s period, 120 req/min → 0.5s period
    let period_seconds = 60.0 / config.requests_per_minute as f64;
    let period = Duration::from_secs_f64(period_seconds);

    info!(
        "creating rate limit layer: {} requests/minute (period: {:.3}s), burst size: {}",
        config.requests_per_minute, period_seconds, config.burst_size
    );

    let governor_config = GovernorConfigBuilder::default()
        .period(period)
        .burst_size(config.burst_size)
        .key_extractor(IpKeyExtractor)
        .finish()
        .expect("failed to build rate limit config");

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
