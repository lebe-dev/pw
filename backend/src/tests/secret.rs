use chrono::{Days, Local};

use core::secret::{Secret, SecretDownloadPolicy, SecretTTL};

use crate::secret::storage::SecretEntity;
use crate::tests::get_random_string;

pub fn get_sample_secret() -> Secret {
    Secret {
        id: get_random_string(),
        payload: get_random_string(),
        ttl: SecretTTL::OneHour,
        download_policy: SecretDownloadPolicy::Unlimited
    }
}

pub fn get_sample_expired_secret_entity() -> SecretEntity {
    let expired = Local::now().checked_sub_days(Days::new(100)).unwrap();

    let mut secret = get_sample_secret();
    secret.ttl = SecretTTL::OneHour;
    secret.download_policy = SecretDownloadPolicy::Unlimited;

    SecretEntity {
        secret: secret.clone(),
        created: expired.timestamp(),
    }
}