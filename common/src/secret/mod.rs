use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

pub mod id;
pub mod key;
pub mod url;

#[derive(Serialize,Deserialize,PartialEq,Clone,Debug)]
#[serde(rename_all = "camelCase")]
pub struct Secret {
    pub id: String,

    /// Data encrypted on frontend side
    pub payload: String,

    pub ttl: SecretTTL,

    pub download_policy: SecretDownloadPolicy
}

impl Display for Secret {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f, "[Secret] id: '{}', payload: '<encrypted>', ttl: {:?}, download-policy: {:?} [/Secret]",
            self.id, self.ttl, self.download_policy
        )
    }
}

#[derive(Serialize,Deserialize,PartialEq,Clone,Debug)]
#[serde(rename_all = "PascalCase")]
pub enum SecretTTL {
    OneHour,
    TwoHours,
    OneDay
}

#[derive(Serialize,Deserialize,PartialEq,Clone,Debug)]
#[serde(rename_all = "PascalCase")]
pub enum SecretDownloadPolicy {
    OneTime,
    Unlimited
}