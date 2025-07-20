use crate::AppState;
use crate::dto::model::AppConfigDto;
use crate::middleware::client_ip::ClientIp;
use axum::Json;
use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use log::info;
use std::sync::Arc;

pub async fn get_config_route(
    State(state): State<Arc<AppState>>,
    request: Request,
) -> impl IntoResponse {
    let client_ip = request
        .extensions()
        .get::<ClientIp>()
        .map(|ip| ip.0.to_string())
        .unwrap_or_else(|| "127.0.0.1".to_string());

    let limits = state.limits_service.get_limits_for_ip(&client_ip);

    info!(
        "Config request from {}: returning limits message_max_length: {}, file_max_size: {}",
        client_ip, limits.message_max_length, limits.file_max_size
    );

    let config = AppConfigDto {
        message_max_length: limits.message_max_length,
        file_upload_enabled: state.config.file_upload_enabled,
        file_max_size: limits.file_max_size,
    };

    (StatusCode::OK, Json(config)).into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::model::{AppConfig, IpLimitEntry, IpLimitsConfig};
    use crate::limits::LimitsService;
    use crate::middleware::client_ip::ClientIp;
    use crate::secret::storage::RedisSecretStorage;
    use axum::http::Request as HttpRequest;
    use std::net::IpAddr;
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
        let secret_storage = RedisSecretStorage::new("redis://localhost");

        Arc::new(AppState {
            config,
            limits_service,
            secret_storage,
        })
    }

    fn create_request_with_ip(ip: IpAddr) -> HttpRequest<axum::body::Body> {
        let mut request = HttpRequest::builder()
            .uri("/api/config")
            .body(axum::body::Body::empty())
            .unwrap();

        request.extensions_mut().insert(ClientIp(ip));
        request
    }

    #[tokio::test]
    async fn test_config_route_default_limits() {
        let state = create_test_app_state(None);
        let request = create_request_with_ip("192.168.1.100".parse().unwrap());

        let response = get_config_route(State(state), request).await;
        let response = response.into_response();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let config: AppConfigDto = serde_json::from_slice(&body).unwrap();

        assert_eq!(config.message_max_length, 1024);
        assert_eq!(config.file_max_size, 10485760);
        assert!(config.file_upload_enabled);
    }

    #[tokio::test]
    async fn test_config_route_with_ip_limits_exact_match() {
        let ip_limits = IpLimitsConfig {
            enabled: true,
            whitelist: vec![IpLimitEntry {
                ip: "192.168.1.100".to_string(),
                message_max_length: Some(8192),
                file_max_size: Some(104857600),
            }],
        };

        let state = create_test_app_state(Some(ip_limits));
        let request = create_request_with_ip("192.168.1.100".parse().unwrap());

        let response = get_config_route(State(state), request).await;
        let response = response.into_response();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let config: AppConfigDto = serde_json::from_slice(&body).unwrap();

        assert_eq!(config.message_max_length, 8192);
        assert_eq!(config.file_max_size, 104857600);
        assert!(config.file_upload_enabled);
    }

    #[tokio::test]
    async fn test_config_route_with_ip_limits_cidr_match() {
        let ip_limits = IpLimitsConfig {
            enabled: true,
            whitelist: vec![IpLimitEntry {
                ip: "192.168.0.0/16".to_string(),
                message_max_length: Some(4096),
                file_max_size: Some(52428800),
            }],
        };

        let state = create_test_app_state(Some(ip_limits));
        let request = create_request_with_ip("192.168.1.100".parse().unwrap());

        let response = get_config_route(State(state), request).await;
        let response = response.into_response();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let config: AppConfigDto = serde_json::from_slice(&body).unwrap();

        assert_eq!(config.message_max_length, 4096);
        assert_eq!(config.file_max_size, 52428800);
    }

    #[tokio::test]
    async fn test_config_route_with_ip_limits_no_match() {
        let ip_limits = IpLimitsConfig {
            enabled: true,
            whitelist: vec![IpLimitEntry {
                ip: "10.0.0.0/8".to_string(),
                message_max_length: Some(8192),
                file_max_size: Some(104857600),
            }],
        };

        let state = create_test_app_state(Some(ip_limits));
        let request = create_request_with_ip("192.168.1.100".parse().unwrap());

        let response = get_config_route(State(state), request).await;
        let response = response.into_response();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let config: AppConfigDto = serde_json::from_slice(&body).unwrap();

        // Should return default limits
        assert_eq!(config.message_max_length, 1024);
        assert_eq!(config.file_max_size, 10485760);
    }

    #[tokio::test]
    async fn test_config_route_with_disabled_ip_limits() {
        let ip_limits = IpLimitsConfig {
            enabled: false,
            whitelist: vec![IpLimitEntry {
                ip: "192.168.1.100".to_string(),
                message_max_length: Some(8192),
                file_max_size: Some(104857600),
            }],
        };

        let state = create_test_app_state(Some(ip_limits));
        let request = create_request_with_ip("192.168.1.100".parse().unwrap());

        let response = get_config_route(State(state), request).await;
        let response = response.into_response();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let config: AppConfigDto = serde_json::from_slice(&body).unwrap();

        // Should return default limits despite matching IP
        assert_eq!(config.message_max_length, 1024);
        assert_eq!(config.file_max_size, 10485760);
    }

    #[tokio::test]
    async fn test_config_route_missing_client_ip_extension() {
        let state = create_test_app_state(None);
        let request = HttpRequest::builder()
            .uri("/api/config")
            .body(axum::body::Body::empty())
            .unwrap();

        let response = get_config_route(State(state), request).await;
        let response = response.into_response();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let config: AppConfigDto = serde_json::from_slice(&body).unwrap();

        // Should use default fallback IP (127.0.0.1) and return default limits
        assert_eq!(config.message_max_length, 1024);
        assert_eq!(config.file_max_size, 10485760);
    }

    #[tokio::test]
    async fn test_config_route_with_ipv6_address() {
        let ip_limits = IpLimitsConfig {
            enabled: true,
            whitelist: vec![IpLimitEntry {
                ip: "2001:db8::/32".to_string(),
                message_max_length: Some(16384),
                file_max_size: Some(209715200),
            }],
        };

        let state = create_test_app_state(Some(ip_limits));
        let request = create_request_with_ip("2001:db8::1".parse().unwrap());

        let response = get_config_route(State(state), request).await;
        let response = response.into_response();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let config: AppConfigDto = serde_json::from_slice(&body).unwrap();

        assert_eq!(config.message_max_length, 16384);
        assert_eq!(config.file_max_size, 209715200);
    }

    #[tokio::test]
    async fn test_config_route_with_file_upload_disabled() {
        let base_config = AppConfig {
            listen: "0.0.0.0:8080".to_string(),
            log_level: "info".to_string(),
            log_target: "stdout".to_string(),
            message_max_length: 1024,
            file_upload_enabled: false, // Disabled
            file_max_size: 10485760,
            encrypted_message_max_length: Some(15485760),
            redis_url: "redis://localhost".to_string(),
            ip_limits: None,
        };

        let limits_service = LimitsService::new(&base_config);
        let secret_storage = RedisSecretStorage::new("redis://localhost");

        let state = Arc::new(AppState {
            config: base_config,
            limits_service,
            secret_storage,
        });

        let request = create_request_with_ip("192.168.1.100".parse().unwrap());

        let response = get_config_route(State(state), request).await;
        let response = response.into_response();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let config: AppConfigDto = serde_json::from_slice(&body).unwrap();

        assert!(!config.file_upload_enabled);
        assert_eq!(config.message_max_length, 1024);
        assert_eq!(config.file_max_size, 10485760);
    }
}
