use anyhow::anyhow;
use log::error;

use common::secret::Secret;

use crate::secret::storage::RedisSecretStorage;

pub fn store_secret(secret_storage: &RedisSecretStorage,
                    secret: &Secret, payload_max_length: u16) -> anyhow::Result<()> {

    let mut payload = secret.payload.to_string();

    if payload.len() <= payload_max_length as usize {

        payload.truncate(payload_max_length as usize);

        let new_secret = Secret {
            id: secret.id.to_string(),
            payload: payload.to_string(),
            ttl: secret.ttl.clone(),
            download_policy: secret.download_policy.clone(),
        };

        secret_storage.store(&secret.id, &new_secret)?;

        Ok(())

    } else {
        error!("payload length ({}) is bigger than allowed {}", payload.len(), payload_max_length);
        Err(anyhow!("payload length is bigger than allowed"))
    }
}

#[cfg(test)]
mod tests {
    use common::secret::{SecretDownloadPolicy, SecretTTL};
    use common::tests::get_random_string;

    use crate::secret::storage::{DEFAULT_REDIS_CNN_URL, RedisSecretStorage};
    use crate::secret::usecase::store_secret;
    use crate::tests::secret::get_sample_secret;

    #[ignore]
    #[test]
    fn valid_payload_length_test() {
        let secret_storage = RedisSecretStorage::new(DEFAULT_REDIS_CNN_URL);

        let mut secret = get_sample_secret();
        secret.payload = get_random_string();
        secret.download_policy = SecretDownloadPolicy::Unlimited;
        secret.ttl = SecretTTL::OneDay;

        assert!(store_secret(&secret_storage, &secret, 3000).is_ok());
    }

    #[ignore]
    #[test]
    fn return_error_for_too_large_payload() {
        let secret_storage = RedisSecretStorage::new(DEFAULT_REDIS_CNN_URL);

        let mut secret = get_sample_secret();
        secret.payload = get_random_string();
        secret.download_policy = SecretDownloadPolicy::Unlimited;
        secret.ttl = SecretTTL::OneDay;

        assert!(store_secret(&secret_storage, &secret, 3).is_err());
    }
}