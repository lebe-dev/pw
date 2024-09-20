use serde::{Deserialize, Serialize};

use crate::locale::Locale;

#[derive(PartialEq, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AppConfigDto {
    pub message_max_length: u16,

    pub locale: Locale
}