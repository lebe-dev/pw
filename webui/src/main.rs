#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::*;
use log::info;

use crate::routes::home::HomePage;
use crate::routes::secret::SecretPage;

pub mod routes;
pub mod config;
pub mod secret;
pub mod components;

pub const APP_VERSION: &str = "1.0.0";


#[derive(Routable, Clone)]
enum Route {
    #[route("/")]
    HomePage {},

    #[route("/secret/:encoded_id")]
    SecretPage { encoded_id: String },
}

fn App(cx: Scope) -> Element {
    render! {
        Router::<Route> {}
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    info!("PW v{APP_VERSION}");

    dioxus_web::launch(App);
}