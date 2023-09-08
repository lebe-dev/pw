use std::path::Path;

use backend::config::load_config_from_file;
use backend::secret::storage::RedisSecretStorage;
use backend::startup::Application;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let config_file = Path::new("pw.yml").to_str().expect("unexpected error");

    let app_config = load_config_from_file(&config_file)?;

    let secret_storage = RedisSecretStorage::new(&app_config.redis_url);

    let app = Application::build(app_config, secret_storage).await?;

    app.run_until_stopped().await?;

    Ok(())
}
