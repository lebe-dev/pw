use crate::config::model::AppConfig;
use crate::routes::secret::{get_secret_route, remove_secret_route, store_secret_route};
use crate::secret::storage::RedisSecretStorage;
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
pub mod logging;
pub mod routes;
pub mod secret;

#[cfg(test)]
pub mod tests;

pub const VERSION: &str = "1.9.2 #1";

static INDEX_HTML: &str = "index.html";

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub secret_storage: RedisSecretStorage,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config_file = Path::new("pw.yml").to_str().expect("unexpected error");

    let app_config = load_config_from_file(&config_file)?;

    let logging_config = get_logging_config(&app_config.log_level, &app_config.log_target);
    log4rs::init_config(logging_config).expect("unable to init logging configuration");

    let secret_storage = RedisSecretStorage::new(&app_config.redis_url);

    let app_state = AppState {
        config: app_config.clone(),
        secret_storage,
    };

    let app = Router::new()
        .route("/api/config", get(get_config_route))
        .route(
            "/api/secret",
            post(store_secret_route).layer(DefaultBodyLimit::disable()),
        )
        .route(
            "/api/secret/{id}",
            get(get_secret_route).delete(remove_secret_route),
        )
        .route("/api/version", get(get_version_route))
        .fallback(static_handler)
        .with_state(Arc::new(app_state));

    let bind = format!("{}", app_config.listen);

    let listener = tokio::net::TcpListener::bind(&bind).await.unwrap();

    println!("PW v{VERSION}");
    println!("URL: http://{bind}");

    axum::serve(listener, app).await.unwrap();

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
