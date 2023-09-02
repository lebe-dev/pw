use std::net::TcpListener;

use actix_cors::Cors;
use actix_web::{HttpResponse, HttpServer, Responder, web};
use actix_web::dev::Server;
use log::info;
use mime_guess::from_path;
use rust_embed::RustEmbed;

use crate::config::AppConfig;
use crate::logging::logging::get_logging_config;
use crate::routes::version_route;

#[derive(RustEmbed)]
#[folder = "./static/"]
struct Asset;

fn handle_embedded_file(path: &str) -> HttpResponse {
    match Asset::get(path) {
        Some(content) => HttpResponse::Ok()
            .content_type(from_path(path).first_or_octet_stream().as_ref())
            .body(content.data.into_owned()),
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub fn get_version() -> String {
        "1.0.0".to_string()
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn build(config: AppConfig) -> Result<Self, anyhow::Error> {
        let logging_config = get_logging_config(&config.log_level);

        match log4rs::init_config(logging_config) {
            Ok(_) => {}
            Err(e) => eprintln!("{}", e)
        }

        info!("config: {:?}", config);

        let address = format!("localhost:{}", config.port);
        let listener = TcpListener::bind(&address)?;
        let port = listener.local_addr().expect("unable to get socket").port();

        let server = run(config, listener).await?;

        Ok(Self { port, server })
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

#[actix_web::get("/")]
async fn index() -> impl Responder {
    handle_embedded_file("index.html")
}

#[actix_web::get("/{_:.*}")]
async fn dist(path: web::Path<String>) -> impl Responder {
    handle_embedded_file(path.as_str())
}

pub async fn run(config: AppConfig,
                 listener: TcpListener) -> anyhow::Result<Server> {

    let app_banner = format!("PW v{}", Application::get_version());
    println!("{app_banner}");
    info!("{app_banner}");

    let server = HttpServer::new(move || {

        let cors = Cors::default()
            .allow_any_header()
            .allow_any_origin()
            .supports_credentials()
            .allowed_methods(vec!["GET", "POST"])
            .max_age(3600);

        actix_web::App::new()
            .wrap(cors)
            .app_data(web::Data::new(config.clone()))
            .service(version_route)
            .service(index)
            .service(dist)

    }).listen(listener)?.run();

    Ok(server)
}