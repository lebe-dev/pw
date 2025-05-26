use crate::VERSION;
use axum::http::StatusCode;
use axum::response::IntoResponse;

pub async fn get_version_route() -> impl IntoResponse {
    (StatusCode::OK, VERSION.to_string()).into_response()
}
