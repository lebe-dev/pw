use std::fmt::{Display, Formatter};

use serde::Deserialize;

#[derive(PartialEq, Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct IpLimitEntry {
    pub ip: String,
    pub message_max_length: Option<u16>,
    pub file_max_size: Option<u64>,
}

#[derive(PartialEq, Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct IpLimitsConfig {
    pub enabled: bool,
    pub whitelist: Vec<IpLimitEntry>,
}

#[derive(PartialEq, Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct AppConfig {
    pub listen: String,

    pub log_level: String,

    pub log_target: String,

    /// Message max length, all above will be truncated
    pub message_max_length: u16,

    pub file_upload_enabled: bool,

    /// File max size
    pub file_max_size: u64,

    /// Encrypted message max length. If not provided, calculated dynamically.
    pub encrypted_message_max_length: Option<u64>,

    pub redis_url: String,

    pub ip_limits: Option<IpLimitsConfig>,
}

impl Display for AppConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "listen: '{}', log-level: {}, log-target: {}, message-max-length: {},\
            file-upload-enabled: {}, file-max-size: {}, encrypted-message-max-length: {:?}, redis-url: '{}', \
            ip-limits: {:?}",
            self.listen,
            self.log_level,
            self.log_target,
            self.message_max_length,
            self.file_upload_enabled,
            self.file_max_size,
            self.encrypted_message_max_length,
            self.redis_url,
            self.ip_limits
        )
    }
}
