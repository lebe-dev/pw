use anyhow::anyhow;
use log::error;

use common::secret::Secret;
use common::secret::storage::SecretStorage;

pub fn store_secret(secret_storage: &impl SecretStorage,
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

        secret_storage.store(&secret.id, &new_secret);

        Ok(())

    } else {
        error!("payload length ({}) is bigger than allowed {}", payload.len(), payload_max_length);
        Err(anyhow!("payload length is bigger than allowed"))
    }
}

#[cfg(test)]
mod tests {
    use mini_moka::sync::Cache;

    use common::secret::{SecretDownloadPolicy, SecretTTL};
    use common::tests::get_random_string;

    use crate::secret::storage::{InMemorySecretStorage, SecretEntity};
    use crate::secret::usecase::store_secret;
    use crate::tests::secret::get_sample_secret;

    #[test]
    fn valid_payload_length_test() {
        let cache: Cache<String, SecretEntity> = Cache::new(10);
        let secret_storage = InMemorySecretStorage::new(cache);

        let mut secret = get_sample_secret();
        secret.payload = get_random_string();
        secret.download_policy = SecretDownloadPolicy::Unlimited;
        secret.ttl = SecretTTL::OneDay;

        assert!(store_secret(&secret_storage, &secret, 3000).is_ok());
    }

    #[test]
    fn return_error_for_too_large_payload() {
        let cache: Cache<String, SecretEntity> = Cache::new(10);
        let secret_storage = InMemorySecretStorage::new(cache);

        let mut secret = get_sample_secret();
        secret.payload = get_random_string();
        secret.download_policy = SecretDownloadPolicy::Unlimited;
        secret.ttl = SecretTTL::OneDay;

        assert!(store_secret(&secret_storage, &secret, 3).is_err());
    }
}