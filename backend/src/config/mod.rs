use std::fmt::{Display, Formatter};

use config::{Config, File};
use log::info;
use serde::Deserialize;

#[derive(PartialEq, Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct AppConfig {
    pub port: u16,

    pub log_level: String,

    pub storage_items_capacity: u32
}

impl Display for AppConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "port: {}, log-level: {}, storage-items-capacity: {}",
               self.port, self.log_level, self.storage_items_capacity)
    }
}

pub fn load_config_from_file(file_path: &str) -> anyhow::Result<AppConfig> {
    info!("load config from file '{file_path}'");

    let settings = Config::builder()
        .add_source(File::with_name(&file_path))
        .build()?;

    let config = settings.try_deserialize::<AppConfig>()?;

    info!("config: {}", config);

    Ok(config)
}

#[cfg(test)]
mod tests {
    #[test]
    fn config_should_be_loaded() {

    }
}