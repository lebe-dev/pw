use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Secret {
    pub id: String,

    pub content_type: SecretContentType,
    pub metadata: SecretFileMetadata,

    /// Data encrypted on frontend side
    pub payload: String,

    pub ttl: SecretTTL,

    pub download_policy: SecretDownloadPolicy,
}

impl Display for Secret {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[Secret] id: '{}', content_type: {:?}, payload: '<encrypted>', ttl: {:?}, download-policy: {:?}, metadata: {:?}, [/Secret]",
            self.id, self.content_type, self.ttl, self.download_policy, self.metadata,
        )
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum SecretContentType {
    Text,
    File,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum SecretTTL {
    OneHour,
    TwoHours,
    OneDay,
    OneWeek,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum SecretDownloadPolicy {
    OneTime,
    Unlimited,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SecretFileMetadata {
    pub name: String,
    pub r#type: String,
    pub size: u64,
}
