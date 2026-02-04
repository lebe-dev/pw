use crate::AppState;
use crate::middleware::client_ip::ClientIp;
use crate::secret::model::{Secret, SecretContentType};
use crate::secret::usecase::store_secret;
use axum::Json;
use axum::extract::{Extension, Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use log::{error, info};
use std::sync::Arc;

pub async fn store_secret_route(
    State(state): State<Arc<AppState>>,
    Extension(client_ip): Extension<ClientIp>,
    secret: Json<Secret>,
) -> StatusCode {
    if secret.content_type == SecretContentType::File && !state.config.file_upload_enabled {
        info!("file upload is disabled");
        return StatusCode::BAD_REQUEST;
    }

    let client_ip_str = client_ip.0.to_string();
    let client_limits = state.limits_service.get_limits_for_ip(&client_ip_str);

    info!(
        "secret storage request from {}: applying encrypted_message_max_length: {}",
        client_ip_str, client_limits.encrypted_message_max_length
    );

    match store_secret(
        state.secret_storage.as_ref(),
        &secret,
        client_limits.encrypted_message_max_length,
    ) {
        Ok(_) => {
            info!("secret stored successfully for client {}", client_ip_str);
            StatusCode::OK
        }
        Err(e) => {
            error!("failed to store secret for client {}: {}", client_ip_str, e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

pub async fn get_secret_route(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match state.secret_storage.load(&id) {
        Ok(secret) => match secret {
            Some(secret) => (StatusCode::OK, Json(secret)).into_response(),
            None => StatusCode::BAD_REQUEST.into_response(),
        },
        Err(e) => {
            error!("{}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn remove_secret_route(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> StatusCode {
    match state.secret_storage.remove(&id) {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            error!("{}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::model::{AppConfig, IpLimitEntry, IpLimitsConfig};
    use crate::limits::LimitsService;
    use crate::metrics::service::MetricsServer;
    use crate::middleware::client_ip::ClientIp;
    use crate::secret::model::{SecretDownloadPolicy, SecretFileMetadata, SecretTTL};
    use crate::secret::storage::MockSecretStorage;
    use std::sync::Arc;

    fn create_test_app_state(
        ip_limits_config: Option<IpLimitsConfig>,
        file_upload_enabled: bool,
    ) -> Arc<AppState> {
        let config = AppConfig {
            listen: "0.0.0.0:8080".to_string(),
            log_level: "info".to_string(),
            log_target: "stdout".to_string(),
            message_max_length: 1024,
            file_upload_enabled,
            file_max_size: 10485760,
            encrypted_message_max_length: Some(15485760),
            redis_url: "redis://localhost".to_string(),
            ip_limits: ip_limits_config,
        };

        let limits_service = LimitsService::new(&config);
        let secret_storage = MockSecretStorage::new();

        let body_limit = limits_service
            .body_limit_as_usize()
            .expect("Failed to calculate body limit");

        let metrics_server = MetricsServer::new(config.clone(), body_limit);

        Arc::new(AppState {
            config,
            limits_service,
            secret_storage: Box::new(secret_storage),
            body_limit,
            metrics_server,
        })
    }

    fn create_test_secret(content_type: SecretContentType, payload_size: usize) -> Secret {
        Secret {
            id: "test-secret-id".to_string(),
            content_type,
            metadata: SecretFileMetadata {
                name: "test.txt".to_string(),
                r#type: "text/plain".to_string(),
                size: payload_size as u64,
            },
            payload: "A".repeat(payload_size), // Simulate encrypted payload
            ttl: SecretTTL::OneHour,
            download_policy: SecretDownloadPolicy::OneTime,
        }
    }

    #[tokio::test]
    async fn test_store_text_secret_with_default_limits() {
        let state = create_test_app_state(None, true);
        let client_ip = ClientIp("192.168.1.100".parse().unwrap());
        let secret = create_test_secret(SecretContentType::Text, 1000); // Within default encrypted limit

        let response = store_secret_route(State(state), Extension(client_ip), Json(secret)).await;

        assert_eq!(response, StatusCode::OK);
    }

    #[tokio::test]
    async fn test_store_text_secret_exceeding_default_limits() {
        let state = create_test_app_state(None, true);
        let client_ip = ClientIp("192.168.1.100".parse().unwrap());
        let secret = create_test_secret(SecretContentType::Text, 20_000_000); // Exceeds default encrypted limit

        let response = store_secret_route(State(state), Extension(client_ip), Json(secret)).await;

        assert_eq!(response, StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_store_secret_with_increased_ip_limits() {
        let ip_limits = IpLimitsConfig {
            enabled: true,
            whitelist: vec![IpLimitEntry {
                ip: "192.168.1.100".to_string(),
                message_max_length: Some(8192), // Increased from 1024
                file_max_size: Some(4096000),   // 4MB instead of 100MB
            }],
            trusted_proxies: vec![],
        };

        let state = create_test_app_state(Some(ip_limits), true);
        let client_ip = ClientIp("192.168.1.100".parse().unwrap());

        // Expected: max(8192, 4096000) * 1.35 = 4096000 * 1.35 = 5529600
        let secret = create_test_secret(SecretContentType::Text, 3_000_000); // Within increased limit (3MB)

        let response = store_secret_route(State(state), Extension(client_ip), Json(secret)).await;

        assert_eq!(response, StatusCode::OK);
    }

    #[tokio::test]
    async fn test_store_secret_with_cidr_match() {
        let ip_limits = IpLimitsConfig {
            enabled: true,
            whitelist: vec![IpLimitEntry {
                ip: "192.168.0.0/16".to_string(),
                message_max_length: Some(4096),
                file_max_size: Some(2048000), // 2MB instead of 50MB
            }],
            trusted_proxies: vec![],
        };

        let state = create_test_app_state(Some(ip_limits), true);
        let client_ip = ClientIp("192.168.1.100".parse().unwrap()); // Matches CIDR

        // Expected: max(4096, 2048000) * 1.35 = 2048000 * 1.35 = 2764800
        let secret = create_test_secret(SecretContentType::Text, 1_500_000); // Within CIDR limit (1.5MB)

        let response = store_secret_route(State(state), Extension(client_ip), Json(secret)).await;

        assert_eq!(response, StatusCode::OK);
    }

    #[tokio::test]
    async fn test_store_secret_with_no_ip_match() {
        let ip_limits = IpLimitsConfig {
            enabled: true,
            whitelist: vec![IpLimitEntry {
                ip: "10.0.0.0/8".to_string(),
                message_max_length: Some(8192),
                file_max_size: Some(104857600),
            }],
            trusted_proxies: vec![],
        };

        let state = create_test_app_state(Some(ip_limits), true);
        let client_ip = ClientIp("192.168.1.100".parse().unwrap()); // Doesn't match whitelist
        let secret = create_test_secret(SecretContentType::Text, 20_000_000); // Exceeds default limit

        let response = store_secret_route(State(state), Extension(client_ip), Json(secret)).await;

        // Should fail because IP doesn't match and falls back to default limits
        assert_eq!(response, StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_store_file_secret_with_upload_enabled() {
        let state = create_test_app_state(None, true);
        let client_ip = ClientIp("192.168.1.100".parse().unwrap());
        let secret = create_test_secret(SecretContentType::File, 1000);

        let response = store_secret_route(State(state), Extension(client_ip), Json(secret)).await;

        assert_eq!(response, StatusCode::OK);
    }

    #[tokio::test]
    async fn test_store_file_secret_with_upload_disabled() {
        let state = create_test_app_state(None, false); // File upload disabled
        let client_ip = ClientIp("192.168.1.100".parse().unwrap());
        let secret = create_test_secret(SecretContentType::File, 1000);

        let response = store_secret_route(State(state), Extension(client_ip), Json(secret)).await;

        assert_eq!(response, StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_store_file_secret_with_ip_limits_and_upload_disabled() {
        let ip_limits = IpLimitsConfig {
            enabled: true,
            whitelist: vec![IpLimitEntry {
                ip: "192.168.1.100".to_string(),
                message_max_length: Some(8192),
                file_max_size: Some(104857600),
            }],
            trusted_proxies: vec![],
        };

        let state = create_test_app_state(Some(ip_limits), false); // File upload disabled
        let client_ip = ClientIp("192.168.1.100".parse().unwrap());
        let secret = create_test_secret(SecretContentType::File, 1000);

        let response = store_secret_route(State(state), Extension(client_ip), Json(secret)).await;

        // Should still be rejected due to global file upload setting
        assert_eq!(response, StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_store_secret_with_disabled_ip_limits() {
        let ip_limits = IpLimitsConfig {
            enabled: false,
            whitelist: vec![IpLimitEntry {
                ip: "192.168.1.100".to_string(),
                message_max_length: Some(8192),
                file_max_size: Some(104857600),
            }],
            trusted_proxies: vec![],
        };

        let state = create_test_app_state(Some(ip_limits), true);
        let client_ip = ClientIp("192.168.1.100".parse().unwrap());
        let secret = create_test_secret(SecretContentType::Text, 20_000_000); // Exceeds default

        let response = store_secret_route(State(state), Extension(client_ip), Json(secret)).await;

        // Should fail because IP limits are disabled, so default limits apply
        assert_eq!(response, StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_store_secret_boundary_encrypted_limit() {
        let ip_limits = IpLimitsConfig {
            enabled: true,
            whitelist: vec![IpLimitEntry {
                ip: "192.168.1.100".to_string(),
                message_max_length: Some(2048),
                file_max_size: Some(1024000), // 1MB instead of 100MB
            }],
            trusted_proxies: vec![],
        };

        let state = create_test_app_state(Some(ip_limits), true);
        let client_ip = ClientIp("192.168.1.100".parse().unwrap());

        // Dynamic calculation: max(2048, 1024000) * 1.35 = 1382400
        let encrypted_limit = (1024000.0 * 1.35) as usize;

        // Test exactly at the limit
        let secret = create_test_secret(SecretContentType::Text, encrypted_limit);

        let response = store_secret_route(
            State(state.clone()),
            Extension(client_ip.clone()),
            Json(secret),
        )
        .await;

        assert_eq!(response, StatusCode::OK);

        // Test just over the limit
        let secret_over = create_test_secret(SecretContentType::Text, encrypted_limit + 1);

        let response_over =
            store_secret_route(State(state), Extension(client_ip), Json(secret_over)).await;

        assert_eq!(response_over, StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_store_secret_with_ipv6_address() {
        let ip_limits = IpLimitsConfig {
            enabled: true,
            whitelist: vec![IpLimitEntry {
                ip: "2001:db8::/32".to_string(),
                message_max_length: Some(16384),
                file_max_size: Some(2048000), // 2MB instead of 200MB
            }],
            trusted_proxies: vec![],
        };

        let state = create_test_app_state(Some(ip_limits), true);
        let client_ip = ClientIp("2001:db8::1".parse().unwrap());

        // Dynamic calculation: max(16384, 2048000) * 1.35 = 2764800
        let secret = create_test_secret(SecretContentType::Text, 1_500_000); // 1.5MB

        let response = store_secret_route(State(state), Extension(client_ip), Json(secret)).await;

        assert_eq!(response, StatusCode::OK);
    }

    #[ignore]
    #[tokio::test]
    async fn test_get_secret_route_existing() {
        let state = create_test_app_state(None, true);

        // First store a secret
        let client_ip = ClientIp("192.168.1.100".parse().unwrap());
        let secret = create_test_secret(SecretContentType::Text, 1000);
        let secret_id = secret.id.clone();

        let store_response =
            store_secret_route(State(state.clone()), Extension(client_ip), Json(secret)).await;
        assert_eq!(store_response, StatusCode::OK);

        // Then try to get it
        let response = get_secret_route(State(state), Path(secret_id)).await;
        let response = response.into_response();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_get_secret_route_nonexistent() {
        let state = create_test_app_state(None, true);

        let response = get_secret_route(State(state), Path("nonexistent-id".to_string())).await;
        let response = response.into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_remove_secret_route_existing() {
        let state = create_test_app_state(None, true);

        // First store a secret
        let client_ip = ClientIp("192.168.1.100".parse().unwrap());
        let secret = create_test_secret(SecretContentType::Text, 1000);
        let secret_id = secret.id.clone();

        let _store_response =
            store_secret_route(State(state.clone()), Extension(client_ip), Json(secret)).await;

        // Then remove it
        let response = remove_secret_route(State(state), Path(secret_id)).await;
        assert_eq!(response, StatusCode::OK);
    }

    #[tokio::test]
    async fn test_remove_secret_route_nonexistent() {
        let state = create_test_app_state(None, true);

        let response = remove_secret_route(State(state), Path("nonexistent-id".to_string())).await;
        assert_eq!(response, StatusCode::OK); // Redis doesn't error on removing non-existent keys
    }
}
