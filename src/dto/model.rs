use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AppConfigDto {
    pub message_max_length: u16,
    pub file_upload_enabled: bool,
    pub file_max_size: u64,
    /// Locales offered to users. Empty means all locales are available.
    pub available_locales: Vec<String>,
}
