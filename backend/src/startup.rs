use std::net::TcpListener;

use actix_cors::Cors;
use actix_plus_static_files::{build_hashmap_from_included_dir, Dir, include_dir, ResourceFiles};
use actix_web::{HttpServer, middleware, web};
use actix_web::dev::Server;
use log::info;

use crate::config::AppConfig;
use crate::logging::logging::get_logging_config;
use crate::routes::{get_config_route, get_version_route};
use crate::routes::secret::{get_secret_route, store_secret_route};
use crate::secret::storage::InMemorySecretStorage;

const STATIC_DIR: Dir = include_dir!("./static");

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub fn get_version() -> String {
        "1.1.0".to_string()
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn build(config: AppConfig,
               secret_storage: InMemorySecretStorage) -> Result<Self, anyhow::Error> {

        let logging_config = get_logging_config(&config.log_level);

        match log4rs::init_config(logging_config) {
            Ok(_) => {}
            Err(e) => eprintln!("{}", e)
        }

        info!("config: {:?}", config);

        let address = format!("0.0.0.0:{}", config.port);
        let listener = TcpListener::bind(&address)?;
        let port = listener.local_addr().expect("unable to get socket").port();

        let server = run(config, secret_storage, listener).await?;

        Ok(Self { port, server })
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub async fn run(config: AppConfig, secret_storage: InMemorySecretStorage,
                 listener: TcpListener) -> anyhow::Result<Server> {

    let app_banner = format!("PW v{}", Application::get_version());
    println!("{app_banner}");
    info!("{app_banner}");

    let hash_map = build_hashmap_from_included_dir(&STATIC_DIR);

    let server = HttpServer::new(move || {

        let cors = Cors::default()
            .allow_any_header()
            .allow_any_origin()
            .supports_credentials()
            .allowed_methods(vec!["GET", "POST"])
            .max_age(3600);

        actix_web::App::new()
            .wrap(cors)
            .wrap(middleware::Compress::default())
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(secret_storage.clone()))
            .service(get_config_route)
            .service(get_secret_route)
            .service(store_secret_route)
            .service(get_version_route)
            .service(ResourceFiles::new(
                "/", hash_map.clone()).resolve_not_found_to_root()
            )

    }).listen(listener)?.run();

    Ok(server)
}