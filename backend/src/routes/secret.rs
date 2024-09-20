use crate::secret::usecase::store_secret;
use crate::secret::Secret;
use crate::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use log::error;
use std::sync::Arc;

pub async fn store_secret_route(
    State(state): State<Arc<AppState>>,
    secret: Json<Secret>) -> StatusCode {

    match store_secret(
        &state.secret_storage, &secret,
        state.config.encrypted_message_max_length) {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR
    }
}

pub async fn get_secret_route(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>) -> impl IntoResponse {

    match state.secret_storage.load(&id) {
        Ok(secret) => {
            match secret {
                Some(secret) => (StatusCode::OK, Json(secret)).into_response(),
                None => StatusCode::BAD_REQUEST.into_response()
            }
        }
        Err(e) => {
            error!("{}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn remove_secret_route(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>) -> StatusCode {

    match state.secret_storage.remove(&id) {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            error!("{}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}