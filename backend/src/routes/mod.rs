use actix_web::{get, HttpResponse, Responder};

use crate::startup::Application;

pub mod secret;

#[get("/api/version")]
pub async fn version_route() -> impl Responder {
    HttpResponse::Ok().body(Application::get_version())
}