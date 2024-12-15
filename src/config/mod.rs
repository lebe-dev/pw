use std::env;
use std::fmt::{Display, Formatter};

use config::{Config, File};
use log::info;
use serde::Deserialize;

#[derive(PartialEq, Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct AppConfig {
    pub listen: String,

    pub log_level: String,

    /// Message max length, all above will be truncated
    pub message_max_length: u16,

    /// Encrypted message max length.
    pub encrypted_message_max_length: u16,

    pub redis_url: String
}

impl Display for AppConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,
               "listen: '{}', log-level: {}, message-max-length: {},\
               encrypted-message-max-length: {}, redis-url: '{}'",
               self.listen, self.log_level, self.message_max_length,
               self.encrypted_message_max_length, self.redis_url)
    }
}

pub fn load_config_from_file(file_path: &str) -> anyhow::Result<AppConfig> {
    info!("load config from file '{file_path}'");

    let config_builder = Config::builder()
        .add_source(
            config::Environment::with_prefix("PW")
                .try_parsing(true)
                .separator("_")
        )
        .add_source(File::with_name(&file_path));

    let settings = config_builder.build()?;

    let config = settings.clone().try_deserialize::<AppConfig>()?;

    let listen = get_env_var("PW_LISTEN").unwrap_or(config.listen.to_string());
    let log_level = get_env_var("PW_LOG_LEVEL").unwrap_or(config.log_level);
    let message_max_length = get_env_var("PW_MESSAGE_MAX_LENGTH").unwrap_or(config.message_max_length.to_string());
    let encrypted_message_max_length = get_env_var("PW_ENCRYPTED_MESSAGE_MAX_LENGTH").unwrap_or(config.encrypted_message_max_length.to_string());
    let redis_url = get_env_var("PW_REDIS_URL").unwrap_or(config.redis_url);

    let config = AppConfig {
        listen: listen.parse()?,
        log_level,
        message_max_length: message_max_length.parse()?,
        encrypted_message_max_length: encrypted_message_max_length.parse()?,
        redis_url
    };

    info!("config: {}", config);

    Ok(config)
}

fn get_env_var(name: &str) -> Option<String> {
    env::var(name).ok()
}