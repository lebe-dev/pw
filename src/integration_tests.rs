#[cfg(test)]
mod tests {
    use crate::config::model::{AppConfig, IpLimitsConfig, IpLimitEntry};
    use crate::limits::LimitsService;
    use crate::middleware::client_ip::ClientIpExtractor;
    use crate::routes::{config::get_config_route, secret::{store_secret_route, get_secret_route}};
    use crate::secret::model::{Secret, SecretContentType, SecretTTL, SecretDownloadPolicy, SecretFileMetadata};
    use crate::secret::storage::MockSecretStorage;
    use crate::dto::model::AppConfigDto;
    use crate::AppState;
    use axum::{
        Router,
        routing::{get, post},
        middleware,
        extract::{ConnectInfo, DefaultBodyLimit},
        http::{Request, StatusCode, HeaderMap},
        body::Body,
    };
    use tower::util::ServiceExt;
    use std::net::SocketAddr;
    use std::sync::Arc;

    fn create_test_app_state(ip_limits_config: Option<IpLimitsConfig>) -> Arc<AppState> {
        let config = AppConfig {
            listen: "0.0.0.0:8080".to_string(),
            log_level: "info".to_string(),
            log_target: "stdout".to_string(),
            message_max_length: 1024,
            file_upload_enabled: true,
            file_max_size: 10485760,
            encrypted_message_max_length: Some(15485760),
            redis_url: "redis://localhost".to_string(),
            ip_limits: ip_limits_config,
        };

        let limits_service = LimitsService::new(&config);
        let secret_storage = MockSecretStorage::new();

        Arc::new(AppState {
            config,
            limits_service,
            secret_storage: Box::new(secret_storage),
        })
    }

    fn create_test_router(app_state: Arc<AppState>) -> Router {
        Router::new()
            .route("/api/config", get(get_config_route))
            .route("/api/secret", post(store_secret_route).layer(DefaultBodyLimit::disable()))
            .route("/api/secret/{id}", get(get_secret_route))
            .layer(middleware::from_fn(ClientIpExtractor::middleware))
            .with_state(app_state)
    }

    fn create_test_secret(content_type: SecretContentType, payload_size: usize) -> Secret {
        Secret {
            id: format!("test-secret-{}", uuid::Uuid::new_v4()),
            content_type,
            metadata: SecretFileMetadata {
                name: "test.txt".to_string(),
                r#type: "text/plain".to_string(),
                size: payload_size as u64,
            },
            payload: "A".repeat(payload_size),
            ttl: SecretTTL::OneHour,
            download_policy: SecretDownloadPolicy::OneTime,
        }
    }

    #[tokio::test]
    async fn test_end_to_end_config_flow_with_default_limits() {
        let app_state = create_test_app_state(None);
        let app = create_test_router(app_state);

        let request = Request::builder()
            .uri("/api/config")
            .extension(ConnectInfo(SocketAddr::from(([192, 168, 1, 100], 8080))))
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let config: AppConfigDto = serde_json::from_slice(&body).unwrap();

        assert_eq!(config.message_max_length, 1024);
        assert_eq!(config.file_max_size, 10485760);
        assert!(config.file_upload_enabled);
    }

    #[tokio::test]
    async fn test_end_to_end_config_flow_with_ip_limits() {
        let ip_limits = IpLimitsConfig {
            enabled: true,
            whitelist: vec![
                IpLimitEntry {
                    ip: "192.168.1.100".to_string(),
                    message_max_length: Some(8192),
                    file_max_size: Some(104857600),
                },
            ],
        };

        let app_state = create_test_app_state(Some(ip_limits));
        let app = create_test_router(app_state);

        let request = Request::builder()
            .uri("/api/config")
            .extension(ConnectInfo(SocketAddr::from(([192, 168, 1, 100], 8080))))
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let config: AppConfigDto = serde_json::from_slice(&body).unwrap();

        assert_eq!(config.message_max_length, 8192);
        assert_eq!(config.file_max_size, 104857600);
    }

    #[tokio::test]
    async fn test_end_to_end_config_flow_with_proxy_headers() {
        let ip_limits = IpLimitsConfig {
            enabled: true,
            whitelist: vec![
                IpLimitEntry {
                    ip: "203.0.113.195".to_string(),
                    message_max_length: Some(16384),
                    file_max_size: Some(209715200),
                },
            ],
        };

        let app_state = create_test_app_state(Some(ip_limits));
        let app = create_test_router(app_state);

        let mut headers = HeaderMap::new();
        headers.insert("x-forwarded-for", "203.0.113.195, 192.168.1.1".parse().unwrap());

        let request = Request::builder()
            .uri("/api/config")
            .extension(ConnectInfo(SocketAddr::from(([192, 168, 1, 1], 8080))))
            .body(Body::empty())
            .unwrap();

        let (mut parts, body) = request.into_parts();
        parts.headers = headers;
        let request = Request::from_parts(parts, body);

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let config: AppConfigDto = serde_json::from_slice(&body).unwrap();

        // Should use limits for 203.0.113.195 (from X-Forwarded-For header)
        assert_eq!(config.message_max_length, 16384);
        assert_eq!(config.file_max_size, 209715200);
    }

    #[tokio::test]
    async fn test_end_to_end_secret_storage_with_default_limits() {
        let app_state = create_test_app_state(None);
        let app = create_test_router(app_state);

        let secret = create_test_secret(SecretContentType::Text, 1000); // Within default limits

        let request = Request::builder()
            .uri("/api/secret")
            .method("POST")
            .extension(ConnectInfo(SocketAddr::from(([192, 168, 1, 100], 8080))))
            .header("content-type", "application/json")
            .body(Body::from(serde_json::to_string(&secret).unwrap()))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_end_to_end_secret_storage_exceeding_default_limits() {
        let app_state = create_test_app_state(None);
        let app = create_test_router(app_state);

        let secret = create_test_secret(SecretContentType::Text, 20_000_000); // Exceeds default encrypted limit

        let request = Request::builder()
            .uri("/api/secret")
            .method("POST")
            .extension(ConnectInfo(SocketAddr::from(([192, 168, 1, 100], 8080))))
            .header("content-type", "application/json")
            .body(Body::from(serde_json::to_string(&secret).unwrap()))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_end_to_end_secret_storage_with_increased_ip_limits() {
        let ip_limits = IpLimitsConfig {
            enabled: true,
            whitelist: vec![
                IpLimitEntry {
                    ip: "192.168.1.100".to_string(),
                    message_max_length: Some(8192), // 8x default
                    file_max_size: Some(4096000), // 4MB instead of 100MB
                },
            ],
        };

        let app_state = create_test_app_state(Some(ip_limits));
        let app = create_test_router(app_state);

        // Dynamic calculation: max(8192, 4096000) * 1.35 = 5529600
        let secret = create_test_secret(SecretContentType::Text, 3_000_000); // Within increased limit (3MB)

        let request = Request::builder()
            .uri("/api/secret")
            .method("POST")
            .extension(ConnectInfo(SocketAddr::from(([192, 168, 1, 100], 8080))))
            .header("content-type", "application/json")
            .body(Body::from(serde_json::to_string(&secret).unwrap()))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_end_to_end_cidr_matching() {
        let ip_limits = IpLimitsConfig {
            enabled: true,
            whitelist: vec![
                IpLimitEntry {
                    ip: "192.168.0.0/16".to_string(),
                    message_max_length: Some(4096),
                    file_max_size: Some(52428800),
                },
            ],
        };

        let app_state = create_test_app_state(Some(ip_limits));

        // Test multiple IPs within the CIDR range
        let test_ips = vec![
            ([192, 168, 1, 100], true),   // Should match
            ([192, 168, 255, 1], true),  // Should match
            ([192, 167, 1, 100], false), // Should not match
            ([193, 168, 1, 100], false), // Should not match
        ];

        for (ip_octets, should_match) in test_ips {
            let app = create_test_router(app_state.clone());

            let request = Request::builder()
                .uri("/api/config")
                .extension(ConnectInfo(SocketAddr::from((ip_octets, 8080))))
                .body(Body::empty())
                .unwrap();

            let response = app.oneshot(request).await.unwrap();
            assert_eq!(response.status(), StatusCode::OK);

            let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
            let config: AppConfigDto = serde_json::from_slice(&body).unwrap();

            if should_match {
                assert_eq!(config.message_max_length, 4096, "Failed for IP: {:?}", ip_octets);
                assert_eq!(config.file_max_size, 52428800, "Failed for IP: {:?}", ip_octets);
            } else {
                assert_eq!(config.message_max_length, 1024, "Failed for IP: {:?}", ip_octets); // default
                assert_eq!(config.file_max_size, 10485760, "Failed for IP: {:?}", ip_octets); // default
            }
        }
    }

    #[tokio::test]
    async fn test_end_to_end_header_precedence() {
        let ip_limits = IpLimitsConfig {
            enabled: true,
            whitelist: vec![
                IpLimitEntry {
                    ip: "203.0.113.195".to_string(), // From X-Forwarded-For
                    message_max_length: Some(8192),
                    file_max_size: Some(104857600),
                },
                IpLimitEntry {
                    ip: "198.51.100.178".to_string(), // From X-Real-IP
                    message_max_length: Some(4096),
                    file_max_size: Some(52428800),
                },
            ],
        };

        let app_state = create_test_app_state(Some(ip_limits));
        let app = create_test_router(app_state);

        let mut headers = HeaderMap::new();
        headers.insert("x-forwarded-for", "203.0.113.195".parse().unwrap());
        headers.insert("x-real-ip", "198.51.100.178".parse().unwrap());

        let request = Request::builder()
            .uri("/api/config")
            .extension(ConnectInfo(SocketAddr::from(([192, 168, 1, 1], 8080))))
            .body(Body::empty())
            .unwrap();

        let (mut parts, body) = request.into_parts();
        parts.headers = headers;
        let request = Request::from_parts(parts, body);

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let config: AppConfigDto = serde_json::from_slice(&body).unwrap();

        // Should use X-Forwarded-For (takes precedence)
        assert_eq!(config.message_max_length, 8192);
        assert_eq!(config.file_max_size, 104857600);
    }

    #[tokio::test]
    async fn test_end_to_end_invalid_headers_fallback() {
        let ip_limits = IpLimitsConfig {
            enabled: true,
            whitelist: vec![
                IpLimitEntry {
                    ip: "192.168.1.1".to_string(), // Connection IP
                    message_max_length: Some(2048),
                    file_max_size: Some(26214400),
                },
            ],
        };

        let app_state = create_test_app_state(Some(ip_limits));
        let app = create_test_router(app_state);

        let mut headers = HeaderMap::new();
        headers.insert("x-forwarded-for", "invalid-ip".parse().unwrap());
        headers.insert("x-real-ip", "also-invalid".parse().unwrap());

        let request = Request::builder()
            .uri("/api/config")
            .extension(ConnectInfo(SocketAddr::from(([192, 168, 1, 1], 8080))))
            .body(Body::empty())
            .unwrap();

        let (mut parts, body) = request.into_parts();
        parts.headers = headers;
        let request = Request::from_parts(parts, body);

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let config: AppConfigDto = serde_json::from_slice(&body).unwrap();

        // Should fall back to connection IP and use its limits
        assert_eq!(config.message_max_length, 2048);
        assert_eq!(config.file_max_size, 26214400);
    }

    #[tokio::test]
    async fn test_end_to_end_complete_secret_lifecycle() {
        let ip_limits = IpLimitsConfig {
            enabled: true,
            whitelist: vec![
                IpLimitEntry {
                    ip: "192.168.1.100".to_string(),
                    message_max_length: Some(8192),
                    file_max_size: Some(4096000), // 4MB instead of 100MB
                },
            ],
        };

        let app_state = create_test_app_state(Some(ip_limits));

        // Step 1: Check config
        let app = create_test_router(app_state.clone());
        let config_request = Request::builder()
            .uri("/api/config")
            .extension(ConnectInfo(SocketAddr::from(([192, 168, 1, 100], 8080))))
            .body(Body::empty())
            .unwrap();

        let config_response = app.oneshot(config_request).await.unwrap();
        assert_eq!(config_response.status(), StatusCode::OK);

        let config_body = axum::body::to_bytes(config_response.into_body(), usize::MAX).await.unwrap();
        let config: AppConfigDto = serde_json::from_slice(&config_body).unwrap();
        assert_eq!(config.message_max_length, 8192);

        // Step 2: Store secret
        let app = create_test_router(app_state.clone());
        let secret = create_test_secret(SecretContentType::Text, 3_000_000); // Within increased limit (3MB)
        let secret_id = secret.id.clone();

        let store_request = Request::builder()
            .uri("/api/secret")
            .method("POST")
            .extension(ConnectInfo(SocketAddr::from(([192, 168, 1, 100], 8080))))
            .header("content-type", "application/json")
            .body(Body::from(serde_json::to_string(&secret).unwrap()))
            .unwrap();

        let store_response = app.oneshot(store_request).await.unwrap();
        assert_eq!(store_response.status(), StatusCode::OK);

        // Step 3: Retrieve secret
        let app = create_test_router(app_state);
        let get_request = Request::builder()
            .uri(&format!("/api/secret/{}", secret_id))
            .extension(ConnectInfo(SocketAddr::from(([192, 168, 1, 100], 8080))))
            .body(Body::empty())
            .unwrap();

        let get_response = app.oneshot(get_request).await.unwrap();
        assert_eq!(get_response.status(), StatusCode::OK);

        let secret_body = axum::body::to_bytes(get_response.into_body(), usize::MAX).await.unwrap();
        let retrieved_secret: Secret = serde_json::from_slice(&secret_body).unwrap();
        assert_eq!(retrieved_secret.id, secret_id);
    }
}