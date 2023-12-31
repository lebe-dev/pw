use std::fmt::{Display, Formatter};

use config::{Config, File};
use log::{error, info};
use serde::Deserialize;
use walkdir::WalkDir;

use common::locale::Locale;

#[derive(PartialEq, Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct AppConfig {
    pub port: u16,

    pub log_level: String,

    /// Message max length, all above will be truncated
    pub message_max_length: u16,

    /// Encrypted message max length.
    pub encrypted_message_max_length: u16,

    pub locale_id: String,

    #[serde(default = "get_default_locales")]
    pub locales: Vec<Locale>,

    pub redis_url: String
}

fn get_default_locales() -> Vec<Locale> {
    vec![]
}

impl Display for AppConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,
               "port: {}, log-level: {}, message-max-length: {},\
               encrypted-message-max-length: {}, locale-id: '{}', redis-url: '{}'",
               self.port, self.log_level, self.message_max_length,
               self.encrypted_message_max_length, self.locale_id, self.redis_url)
    }
}

pub fn load_config_from_file(file_path: &str) -> anyhow::Result<AppConfig> {
    info!("load config from file '{file_path}'");

    let config_builder = Config::builder()
        .add_source(File::with_name(&file_path));

    let settings = config_builder.build()?;

    let config = settings.try_deserialize::<AppConfig>()?;

    let locales = load_locales_from_files("locale.d")?;

    let config = AppConfig {
        port: config.port,
        log_level: config.log_level,
        message_max_length: config.message_max_length,
        encrypted_message_max_length: config.encrypted_message_max_length,
        locale_id: config.locale_id,
        locales,
        redis_url: config.redis_url
    };

    info!("config: {}", config);

    Ok(config)
}

pub fn load_locales_from_files(path: &str) -> anyhow::Result<Vec<Locale>> {
    info!("load locales from path '{path}'");

    let mut locales: Vec<Locale> = vec![];

    for entry in WalkDir::new(path) {
        let entry = entry?;

        let metadata = entry.metadata()?;

        if metadata.is_file() {

            if let Some(filename) = entry.file_name().to_str() {
                if filename.ends_with("yml") {
                    let path = entry.path().clone();
                    let path = format!("{}", path.display());

                    match load_locale_from_file(&path) {
                        Ok(locale) => locales.push(locale.clone()),
                        Err(e) => error!("locale '{}' load error: {}", path, e)
                    }
                }
            }

        }
    }

    Ok(locales)
}

pub fn load_locale_from_file(file_path: &str) -> anyhow::Result<Locale> {
    let config_builder = Config::builder()
        .add_source(File::with_name(&file_path));

    let settings = config_builder.build()?;

    let locale = settings.try_deserialize::<Locale>()?;

    info!("locale: {:?}", locale);

    Ok(locale)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use common::locale::Locale;

    use crate::config::{load_locale_from_file, load_locales_from_files};

    #[test]
    fn load_locales_from_path() {
        let path = Path::new("test-data");
        let path = format!("{}", path.display());
        let locales = load_locales_from_files(&path).unwrap();
        assert_eq!(2, locales.len());
    }

    #[test]
    fn load_locale_from_file_test() {
        let path = Path::new("test-data").join("en.yml");
        let locale = load_locale_from_file(&path.to_str().unwrap()).unwrap();

        let expected_locale = Locale::default();

        assert_eq!(expected_locale, locale);
    }
}