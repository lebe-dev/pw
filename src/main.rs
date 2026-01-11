use crate::config::model::AppConfig;
use crate::routes::secret::{get_secret_route, remove_secret_route, store_secret_route};
use crate::secret::storage::{RedisSecretStorage, SecretStorage};
use axum::Router;
use axum::extract::DefaultBodyLimit;
use axum::http::{StatusCode, Uri, header};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, post};
use config::file::load_config_from_file;
use logging::get_logging_config;
use routes::config::get_config_route;
use routes::version::get_version_route;
use rust_embed::Embed;
use std::path::Path;
use std::sync::Arc;

pub mod config;
pub mod dto;
pub mod limits;
pub mod logging;
pub mod middleware;
pub mod routes;
pub mod secret;

#[cfg(test)]
pub mod tests;

#[cfg(test)]
pub mod integration_tests;

#[cfg(test)]
pub mod security_tests;

pub const VERSION: &str = "1.12.0 #1";

static INDEX_HTML: &str = "index.html";

pub struct AppState {
    pub config: AppConfig,
    pub secret_storage: Box<dyn SecretStorage + Send + Sync>,
    pub limits_service: limits::LimitsService,
    pub body_limit: usize,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config_file = Path::new("pw.yml").to_str().expect("unexpected error");

    let app_config = load_config_from_file(config_file)?;

    let logging_config = get_logging_config(&app_config.log_level, &app_config.log_target);
    log4rs::init_config(logging_config).expect("unable to init logging configuration");

    let secret_storage = RedisSecretStorage::new(&app_config.redis_url);
    let limits_service = limits::LimitsService::new(&app_config);

    let body_limit = limits_service
        .body_limit_as_usize()
        .expect("failed to calculate body limit");

    log::info!(
        "configured HTTP body limit: {} bytes ({:.2} MB)",
        body_limit,
        body_limit as f64 / 1_048_576.0
    );

    if let Some(ref rate_limit_cfg) = app_config.rate_limit {
        if rate_limit_cfg.enabled {
            log::info!(
                "rate limiting enabled: {} req/min (burst: {})",
                rate_limit_cfg.requests_per_minute,
                rate_limit_cfg.burst_size
            );
        }
    }

    let app_state = AppState {
        config: app_config.clone(),
        secret_storage: Box::new(secret_storage),
        limits_service,
        body_limit,
    };

    let body_limit_for_route = app_state.body_limit;

    let mut app = Router::new()
        .route("/api/config", get(get_config_route))
        .route(
            "/api/secret",
            post(store_secret_route).layer(DefaultBodyLimit::max(body_limit_for_route)),
        )
        .route(
            "/api/secret/{id}",
            get(get_secret_route).delete(remove_secret_route),
        )
        .route("/api/version", get(get_version_route))
        .fallback(static_handler);

    if let Some(ref rate_limit_cfg) = app_config.rate_limit {
        if rate_limit_cfg.enabled {
            app = app.layer(middleware::rate_limit::create_rate_limit_layer(
                rate_limit_cfg,
            ));

            app = app.layer(axum::middleware::from_fn(
                middleware::rate_limit::rate_limit_middleware,
            ));
        }
    }

    let app = app
        .layer(axum::Extension(app_config.ip_limits.clone()))
        .layer(axum::middleware::from_fn(
            middleware::ClientIpExtractor::middleware,
        ))
        .with_state(Arc::new(app_state));

    let bind = app_config.listen.to_string();

    let listener = tokio::net::TcpListener::bind(&bind).await.unwrap();

    println!("PW v{VERSION}");
    println!("URL: http://{bind}");

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .await
    .unwrap();

    Ok(())
}

async fn index_html() -> Response {
    match Assets::get(INDEX_HTML) {
        Some(content) => Html(content.data).into_response(),
        None => not_found().await,
    }
}

async fn static_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');

    if path.is_empty() || path == INDEX_HTML {
        return index_html().await;
    }

    match Assets::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();

            ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
        }
        None => {
            if path.contains('.') {
                return not_found().await;
            }

            index_html().await
        }
    }
}

async fn not_found() -> Response {
    (StatusCode::NOT_FOUND, "404").into_response()
}

#[derive(Embed)]
#[folder = "static/"]
struct Assets;
