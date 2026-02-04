use crate::AppState;
use axum::extract::State;
use axum::http::{StatusCode, header};
use axum::response::IntoResponse;
use std::sync::Arc;

pub async fn get_metrics_route(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let metrics = state.metrics_server.get_metrics().await;
    let body = metrics.to_prometheus_text();

    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/plain; version=0.0.4")],
        body,
    )
}
