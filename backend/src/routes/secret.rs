use actix_web::{get, HttpResponse, post, Responder, web};
use log::error;

use common::secret::Secret;

use crate::config::AppConfig;
use crate::secret::storage::RedisSecretStorage;
use crate::secret::usecase::store_secret;

#[post("/api/secret")]
pub async fn store_secret_route(
    app_config: web::Data<AppConfig>,
    secret_storage: web::Data<RedisSecretStorage>,
    secret: web::Json<Secret>) -> impl Responder {

    match store_secret(
        secret_storage.as_ref(), &secret,
        app_config.encrypted_message_max_length) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::Forbidden().finish()
    }
}

#[get("/api/secret/{id}")]
pub async fn get_secret_route(
    path: web::Path<(String, )>,
    secret_storage: web::Data<RedisSecretStorage>) -> impl Responder {

    let secret_id = path.into_inner().0;

    match secret_storage.load(&secret_id) {
        Ok(secret) => {
            match secret {
                Some(secret) => HttpResponse::Ok().json(secret),
                None => HttpResponse::BadRequest().finish()
            }
        }
        Err(e) => {
            error!("{}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}