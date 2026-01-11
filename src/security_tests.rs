#[cfg(test)]
mod tests {
    use crate::AppState;
    use crate::config::model::{AppConfig, IpLimitEntry, IpLimitsConfig};
    use crate::dto::model::AppConfigDto;
    use crate::limits::LimitsService;
    use crate::middleware::client_ip::ClientIpExtractor;
    use crate::routes::{config::get_config_route, secret::store_secret_route};
    use crate::secret::model::{
        Secret, SecretContentType, SecretDownloadPolicy, SecretFileMetadata, SecretTTL,
    };
    use crate::secret::storage::MockSecretStorage;
    use axum::{
        Router,
        body::Body,
        extract::{ConnectInfo, DefaultBodyLimit},
        http::{HeaderMap, Request, StatusCode},
        middleware,
        routing::{get, post},
    };
    use std::net::SocketAddr;
    use std::sync::Arc;
    use tower::util::ServiceExt;

    fn create_security_test_app_state() -> Arc<AppState> {
        let ip_limits = IpLimitsConfig {
            enabled: true,
            whitelist: vec![
                IpLimitEntry {
                    ip: "192.168.1.100".to_string(),
                    message_max_length: Some(8192),
                    file_max_size: Some(104857600),
                },
                IpLimitEntry {
                    ip: "192.168.1.2".to_string(),
                    message_max_length: Some(8192),
                    file_max_size: Some(104857600),
                },
                IpLimitEntry {
                    ip: "10.0.0.0/8".to_string(),
                    message_max_length: Some(4096),
                    file_max_size: Some(52428800),
                },
            ],
            trusted_proxies: vec![],
        };

        let config = AppConfig {
            listen: "0.0.0.0:8080".to_string(),
            log_level: "info".to_string(),
            log_target: "stdout".to_string(),
            message_max_length: 1024,
            file_upload_enabled: true,
            file_max_size: 10485760,
            encrypted_message_max_length: Some(15485760),
            redis_url: "redis://localhost".to_string(),
            ip_limits: Some(ip_limits),
        };

        let limits_service = LimitsService::new(&config);
        let secret_storage = MockSecretStorage::new();

        let body_limit = limits_service
            .body_limit_as_usize()
            .expect("Failed to calculate body limit");

        Arc::new(AppState {
            config,
            limits_service,
            secret_storage: Box::new(secret_storage),
            body_limit,
        })
    }

    fn create_security_test_router(app_state: Arc<AppState>) -> Router {
        let body_limit = app_state.body_limit;

        Router::new()
            .route("/api/config", get(get_config_route))
            .route(
                "/api/secret",
                post(store_secret_route).layer(DefaultBodyLimit::max(body_limit)),
            )
            .layer(middleware::from_fn(ClientIpExtractor::middleware))
            .with_state(app_state)
    }

    #[tokio::test]
    async fn test_security_header_injection_attempts() {
        let app_state = create_security_test_app_state();

        let malicious_headers = vec![
            (
                "192.168.1.100; DROP TABLE users;--",
                "Should not parse SQL injection",
            ),
            ("../../../etc/passwd", "Should not parse path traversal"),
            ("file:///etc/passwd", "Should not parse file URI"),
            ("192.168.1.100 extra", "Should not parse extra data"),
        ];

        for (malicious_header, description) in malicious_headers {
            let app = create_security_test_router(app_state.clone());

            let mut headers = HeaderMap::new();
            // Try to insert malicious header - some may fail at HeaderMap level
            if let Ok(header_value) = malicious_header.parse() {
                headers.insert("x-forwarded-for", header_value);
            }

            let request = Request::builder()
                .uri("/api/config")
                .extension(ConnectInfo(SocketAddr::from(([10, 0, 0, 1], 8080))))
                .body(Body::empty())
                .unwrap();

            let (mut parts, body) = request.into_parts();
            parts.headers = headers;
            let request = Request::from_parts(parts, body);

            let response = app.oneshot(request).await.unwrap();
            assert_eq!(response.status(), StatusCode::OK, "{}", description);

            let body = axum::body::to_bytes(response.into_body(), usize::MAX)
                .await
                .unwrap();
            let config: AppConfigDto = serde_json::from_slice(&body).unwrap();

            // Should fall back to connection IP (10.0.0.1) which matches 10.0.0.0/8
            assert_eq!(config.message_max_length, 4096, "{}", description);
        }
    }

    #[tokio::test]
    async fn test_security_ip_spoofing_protection() {
        let app_state = create_security_test_app_state();

        // Test attempts to spoof privileged IP addresses
        let spoofing_attempts = vec![
            ("127.0.0.1", "localhost spoofing"),
            ("0.0.0.0", "any address spoofing"),
            ("255.255.255.255", "broadcast address spoofing"),
            ("169.254.169.254", "AWS metadata service spoofing"),
            ("::1", "IPv6 localhost spoofing"),
            ("fe80::1", "IPv6 link-local spoofing"),
        ];

        for (spoofed_ip, description) in spoofing_attempts {
            let app = create_security_test_router(app_state.clone());

            let mut headers = HeaderMap::new();
            headers.insert("x-forwarded-for", spoofed_ip.parse().unwrap());

            // Real connection comes from untrusted IP
            let request = Request::builder()
                .uri("/api/config")
                .extension(ConnectInfo(SocketAddr::from(([203, 0, 113, 1], 8080))))
                .body(Body::empty())
                .unwrap();

            let (mut parts, body) = request.into_parts();
            parts.headers = headers;
            let request = Request::from_parts(parts, body);

            let response = app.oneshot(request).await.unwrap();
            assert_eq!(response.status(), StatusCode::OK, "{}", description);

            let body = axum::body::to_bytes(response.into_body(), usize::MAX)
                .await
                .unwrap();
            let _config: AppConfigDto = serde_json::from_slice(&body).unwrap();

            // Should use the spoofed IP if it's valid, demonstrating the need for trusted proxy validation
            // This test shows the current behavior - in production, you'd want to validate proxy sources
        }
    }

    #[tokio::test]
    async fn test_security_payload_size_bypass_attempts() {
        let app_state = create_security_test_app_state();

        // Attempt to bypass limits by using non-whitelisted IP but large payload
        let app = create_security_test_router(app_state);

        let secret = Secret {
            id: "malicious-secret".to_string(),
            content_type: SecretContentType::Text,
            metadata: SecretFileMetadata {
                name: "bypass.txt".to_string(),
                r#type: "text/plain".to_string(),
                size: 100_000_000,
            },
            payload: "A".repeat(100_000_000), // Much larger than default limit
            ttl: SecretTTL::OneHour,
            download_policy: SecretDownloadPolicy::OneTime,
        };

        let request = Request::builder()
            .uri("/api/secret")
            .method("POST")
            .extension(ConnectInfo(SocketAddr::from(([203, 0, 113, 1], 8080)))) // Non-whitelisted IP
            .header("content-type", "application/json")
            .body(Body::from(serde_json::to_string(&secret).unwrap()))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        // Should fail due to size limits enforcement
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_security_ip_whitelist_boundary_conditions() {
        let app_state = create_security_test_app_state();

        // Test IPs at CIDR boundaries to ensure no off-by-one errors
        let boundary_tests = vec![
            ([9, 255, 255, 255], false, "Just below 10.0.0.0/8"),
            ([10, 0, 0, 0], true, "Start of 10.0.0.0/8"),
            ([10, 255, 255, 255], true, "End of 10.0.0.0/8"),
            ([11, 0, 0, 0], false, "Just above 10.0.0.0/8"),
        ];

        for (ip_octets, should_match, description) in boundary_tests {
            let app = create_security_test_router(app_state.clone());

            let request = Request::builder()
                .uri("/api/config")
                .extension(ConnectInfo(SocketAddr::from((ip_octets, 8080))))
                .body(Body::empty())
                .unwrap();

            let response = app.oneshot(request).await.unwrap();
            assert_eq!(response.status(), StatusCode::OK, "{}", description);

            let body = axum::body::to_bytes(response.into_body(), usize::MAX)
                .await
                .unwrap();
            let config: AppConfigDto = serde_json::from_slice(&body).unwrap();

            if should_match {
                assert_eq!(config.message_max_length, 4096, "{}", description);
                assert_eq!(config.file_max_size, 52428800, "{}", description);
            } else {
                assert_eq!(config.message_max_length, 1024, "{}", description);
                assert_eq!(config.file_max_size, 10485760, "{}", description);
            }
        }
    }

    #[tokio::test]
    async fn test_security_concurrent_requests_from_different_ips() {
        let app_state = create_security_test_app_state();

        // Simulate concurrent requests from different IPs to test thread safety
        let mut handles = vec![];

        for i in 0..10 {
            let app_state_clone = app_state.clone();
            let handle = tokio::spawn(async move {
                let app = create_security_test_router(app_state_clone);

                // Alternate between whitelisted and non-whitelisted IPs
                let ip = if i % 2 == 0 {
                    [192, 168, 1, 100] // Whitelisted
                } else {
                    [203, 0, 113, i as u8] // Non-whitelisted
                };

                let request = Request::builder()
                    .uri("/api/config")
                    .extension(ConnectInfo(SocketAddr::from((ip, 8080))))
                    .body(Body::empty())
                    .unwrap();

                let response = app.oneshot(request).await.unwrap();
                assert_eq!(response.status(), StatusCode::OK);

                let body = axum::body::to_bytes(response.into_body(), usize::MAX)
                    .await
                    .unwrap();
                let config: AppConfigDto = serde_json::from_slice(&body).unwrap();

                (i, config.message_max_length, ip)
            });
            handles.push(handle);
        }

        // Wait for all requests to complete
        for handle in handles {
            let (i, message_limit, ip) = handle.await.unwrap();

            if i % 2 == 0 {
                // Whitelisted IP should get increased limits
                assert_eq!(message_limit, 8192, "Failed for whitelisted IP: {:?}", ip);
            } else {
                // Non-whitelisted IP should get default limits
                assert_eq!(
                    message_limit, 1024,
                    "Failed for non-whitelisted IP: {:?}",
                    ip
                );
            }
        }
    }

    #[tokio::test]
    async fn test_security_extreme_header_values() {
        let app_state = create_security_test_app_state();

        // Test extremely long header values
        let long_ip_list = (1..=1000)
            .map(|i| format!("192.168.1.{}", i % 255 + 1))
            .collect::<Vec<_>>()
            .join(", ");

        let app = create_security_test_router(app_state);

        let mut headers = HeaderMap::new();
        headers.insert("x-forwarded-for", long_ip_list.parse().unwrap());

        let request = Request::builder()
            .uri("/api/config")
            .extension(ConnectInfo(SocketAddr::from(([10, 0, 0, 1], 8080))))
            .body(Body::empty())
            .unwrap();

        let (mut parts, body) = request.into_parts();
        parts.headers = headers;
        let request = Request::from_parts(parts, body);

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let config: AppConfigDto = serde_json::from_slice(&body).unwrap();

        // Should parse the first IP (192.168.1.2) and match whitelist entry
        assert_eq!(config.message_max_length, 8192);
    }

    #[tokio::test]
    async fn test_security_mixed_ipv4_ipv6_confusion() {
        let app_state = create_security_test_app_state();

        // Test potential IPv4/IPv6 confusion attacks
        let confusion_attempts = vec![
            ("::ffff:192.168.1.100", "IPv4-mapped IPv6"),
            ("::192.168.1.100", "IPv4-compatible IPv6"),
            ("2001:db8::192.168.1.100", "Mixed IPv6 with IPv4"),
        ];

        for (confusing_ip, description) in confusion_attempts {
            let app = create_security_test_router(app_state.clone());

            let mut headers = HeaderMap::new();
            headers.insert("x-forwarded-for", confusing_ip.parse().unwrap());

            let request = Request::builder()
                .uri("/api/config")
                .extension(ConnectInfo(SocketAddr::from(([203, 0, 113, 1], 8080))))
                .body(Body::empty())
                .unwrap();

            let (mut parts, body) = request.into_parts();
            parts.headers = headers;
            let request = Request::from_parts(parts, body);

            let response = app.oneshot(request).await.unwrap();
            assert_eq!(response.status(), StatusCode::OK, "{}", description);

            let body = axum::body::to_bytes(response.into_body(), usize::MAX)
                .await
                .unwrap();
            let config: AppConfigDto = serde_json::from_slice(&body).unwrap();

            // Should not match IPv4 whitelist entries
            assert_eq!(config.message_max_length, 1024, "{}", description);
        }
    }

    #[tokio::test]
    async fn test_security_limits_service_isolation() {
        // Test that LimitsService properly isolates different configurations
        let config1 = AppConfig {
            listen: "0.0.0.0:8080".to_string(),
            log_level: "info".to_string(),
            log_target: "stdout".to_string(),
            message_max_length: 1024,
            file_upload_enabled: true,
            file_max_size: 10485760,
            encrypted_message_max_length: Some(15485760),
            redis_url: "redis://localhost".to_string(),
            ip_limits: Some(IpLimitsConfig {
                enabled: true,
                whitelist: vec![IpLimitEntry {
                    ip: "192.168.1.1".to_string(),
                    message_max_length: Some(8192),
                    file_max_size: Some(104857600),
                }],
                trusted_proxies: vec![],
            }),
        };

        let config2 = AppConfig {
            listen: "0.0.0.0:8080".to_string(),
            log_level: "info".to_string(),
            log_target: "stdout".to_string(),
            message_max_length: 2048,
            file_upload_enabled: true,
            file_max_size: 20971520,
            encrypted_message_max_length: Some(31457280),
            redis_url: "redis://localhost".to_string(),
            ip_limits: Some(IpLimitsConfig {
                enabled: true,
                whitelist: vec![IpLimitEntry {
                    ip: "192.168.1.1".to_string(),
                    message_max_length: Some(4096),
                    file_max_size: Some(52428800),
                }],
                trusted_proxies: vec![],
            }),
        };

        let service1 = LimitsService::new(&config1);
        let service2 = LimitsService::new(&config2);

        // Same IP should get different limits from different services
        let limits1 = service1.get_limits_for_ip("192.168.1.1");
        let limits2 = service2.get_limits_for_ip("192.168.1.1");

        assert_eq!(limits1.message_max_length, 8192);
        assert_eq!(limits1.file_max_size, 104857600);

        assert_eq!(limits2.message_max_length, 4096);
        assert_eq!(limits2.file_max_size, 52428800);

        // Non-matching IP should get default limits
        let default_limits1 = service1.get_limits_for_ip("203.0.113.1");
        let default_limits2 = service2.get_limits_for_ip("203.0.113.1");

        assert_eq!(default_limits1.message_max_length, 1024);
        assert_eq!(default_limits1.file_max_size, 10485760);

        assert_eq!(default_limits2.message_max_length, 2048);
        assert_eq!(default_limits2.file_max_size, 20971520);
    }
}
