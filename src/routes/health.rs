use crate::AppState;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde_json::json;
use std::sync::Arc;

pub async fn get_health_route(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match state.secret_storage.ping() {
        Ok(_) => (StatusCode::OK, Json(json!({"status": "ok"}))).into_response(),
        Err(_) => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({"status": "error"})),
        )
            .into_response(),
    }
}
