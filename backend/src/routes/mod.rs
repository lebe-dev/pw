use actix_web::{get, HttpResponse, Responder, web};

use common::dto::AppConfigDto;

use crate::config::AppConfig;
use crate::startup::Application;

pub mod secret;

#[get("/api/config")]
pub async fn get_config_route(app_config: web::Data<AppConfig>) -> impl Responder {
    let config = AppConfigDto {
        message_max_length: app_config.message_max_length,
    };

    HttpResponse::Ok().json(config)
}

#[get("/api/version")]
pub async fn get_version_route() -> impl Responder {
    HttpResponse::Ok().body(Application::get_version())
}