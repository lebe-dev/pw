use crate::dto::AppConfigDto;
use crate::{AppState, VERSION};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use std::sync::Arc;

pub mod secret;

pub async fn get_config_route(State(state): State<Arc<AppState>>,) -> impl IntoResponse {
    let config = AppConfigDto {
        message_max_length: state.config.message_max_length
    };

    (StatusCode::OK, Json(config)).into_response()
}

pub async fn get_version_route() -> impl IntoResponse {
    (StatusCode::OK, VERSION.to_string()).into_response()
}