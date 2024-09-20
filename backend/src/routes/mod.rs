use crate::dto::AppConfigDto;
use crate::{AppState, VERSION};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use log::error;
use std::sync::Arc;

pub mod secret;

pub async fn get_config_route(State(state): State<Arc<AppState>>,) -> impl IntoResponse {
    let locale_found = state.config.locales.iter()
                                                .find(|l|l.id == state.config.locale_id);

    match locale_found {
        Some(locale) => {
            let config = AppConfigDto {
                message_max_length: state.config.message_max_length,
                locale: locale.clone()
            };

            (StatusCode::OK, Json(config)).into_response()
        }
        None => {
            error!("misconfiguration, locale wasn't found by id '{}'", state.config.locale_id);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn get_version_route() -> impl IntoResponse {
    (StatusCode::OK, VERSION.to_string()).into_response()
}