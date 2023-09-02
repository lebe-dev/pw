use actix_web::{get, HttpResponse, post, Responder, web};

use common::secret::Secret;
use common::secret::storage::SecretStorage;

use crate::secret::storage::InMemorySecretStorage;

#[post("/api/secret")]
pub async fn store_secret_route(
    secret_storage: web::Data<InMemorySecretStorage>,
    secret: web::Json<Secret>) -> impl Responder {

    secret_storage.store(&secret.id, &secret);
    HttpResponse::Ok().finish()
}

#[get("/api/secret/{id}")]
pub async fn get_secret_route(
    path: web::Path<(String, )>,
    secret_storage: web::Data<InMemorySecretStorage>) -> impl Responder {

    let secret_id = path.into_inner().0;

    match secret_storage.load(&secret_id) {
        Some(secret) => HttpResponse::Ok().json(secret),
        None => HttpResponse::BadRequest().finish()
    }
}