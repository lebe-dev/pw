use crate::AppState;
use crate::secret::model::{Secret, SecretContentType};
use crate::secret::usecase::store_secret;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use log::{error, info};
use std::sync::Arc;

pub async fn store_secret_route(
    State(state): State<Arc<AppState>>,
    secret: Json<Secret>,
) -> StatusCode {
    if secret.content_type == SecretContentType::File && !state.config.file_upload_enabled {
        info!("file upload is disabled");
        return StatusCode::BAD_REQUEST;
    }

    match store_secret(
        &state.secret_storage,
        &secret,
        state.config.encrypted_message_max_length,
    ) {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
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
