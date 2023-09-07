use std::path::Path;
use std::time::Duration;

use mini_moka::sync::{Cache, CacheBuilder};

use backend::config::load_config_from_file;
use backend::secret::storage::{InMemorySecretStorage, SecretEntity};
use backend::startup::Application;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let config_file = Path::new("pw.yml").to_str().expect("unexpected error");

    let app_config = load_config_from_file(&config_file)?;

    let cache: Cache<String, SecretEntity> = CacheBuilder::new(
                            app_config.storage_items_capacity as u64
                                    ).time_to_live(Duration::from_secs(60 * 60 * 24))
                                    .build();

    let secret_storage = InMemorySecretStorage::new(cache);

    let app = Application::build(app_config, secret_storage).await?;

    app.run_until_stopped().await?;

    Ok(())
}
