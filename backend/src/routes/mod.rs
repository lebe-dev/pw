use actix_web::{get, web, HttpResponse, Responder};
use log::error;

use crate::config::AppConfig;
use crate::dto::AppConfigDto;
use crate::startup::Application;

pub mod secret;

#[get("/api/config")]
pub async fn get_config_route(app_config: web::Data<AppConfig>) -> impl Responder {
    let locale_found = app_config.locales.iter().find(|l|l.id == app_config.locale_id);

    match locale_found {
        Some(locale) => {
            let config = AppConfigDto {
                message_max_length: app_config.message_max_length,
                locale: locale.clone()
            };

            HttpResponse::Ok().json(config)
        }
        None => {
            error!("misconfiguration, locale wasn't found by id '{}'", app_config.locale_id);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/api/version")]
pub async fn get_version_route() -> impl Responder {
    HttpResponse::Ok().body(Application::get_version())
}