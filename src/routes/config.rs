use crate::AppState;
use crate::dto::model::AppConfigDto;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use std::sync::Arc;

pub async fn get_config_route(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let config = AppConfigDto {
        message_max_length: state.config.message_max_length,
        file_upload_enabled: state.config.file_upload_enabled,
        file_max_size: state.config.file_max_size,
    };

    (StatusCode::OK, Json(config)).into_response()
}
