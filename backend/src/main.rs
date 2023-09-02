use std::path::Path;

use backend::config::load_config_from_file;
use backend::startup::Application;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config_file = Path::new("pw.yml").to_str()
        .expect("unexpected error");

    let app_config = load_config_from_file(&config_file)?;

    let app = Application::build(app_config).await?;

    app.run_until_stopped().await?;

    Ok(())
}
