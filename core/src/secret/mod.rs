use std::fmt::{Display, Formatter};

pub mod storage;

#[derive(PartialEq,Clone,Debug)]
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

#[derive(PartialEq,Clone,Debug)]
pub enum SecretTTL {
    OneHour,
    TwoHours,
    OneDay
}

#[derive(PartialEq,Clone,Debug)]
pub enum SecretDownloadPolicy {
    OneTime,
    Unlimited
}