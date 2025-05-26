use crate::secret::{Secret, SecretDownloadPolicy, SecretTTL};
use anyhow::{Context, anyhow};
use log::{debug, error, info};
use redis::{Commands, ExistenceCheck, SetExpiry, SetOptions};

pub const DEFAULT_REDIS_CNN_URL: &str = "redis://127.0.0.1";

#[derive(Clone)]
pub struct RedisSecretStorage {
    cnn_url: String,
}

impl RedisSecretStorage {
    /// For example: `redis://127.0.0.1/`
    pub fn new(cnn_url: &str) -> RedisSecretStorage {
        RedisSecretStorage {
            cnn_url: cnn_url.to_string(),
        }
    }

    pub fn store(&self, id: &str, secret: &Secret) -> anyhow::Result<()> {
        info!("store secret: {}", secret);
        debug!("cnn url: {}", self.cnn_url);
        let client = redis::Client::open(&*self.cnn_url)?;
        let mut cnn = client
            .get_connection()
            .context("couldn't connect to redis")?;

        let ttl_seconds = match secret.ttl {
            SecretTTL::OneHour => 60 * 60,
            SecretTTL::TwoHours => 60 * 60 * 2,
            SecretTTL::OneDay => 60 * 60 * 24,
            SecretTTL::OneWeek => 60 * 60 * 24 * 7,
        };

        debug!("ttl seconds: {ttl_seconds}");

        let opts = SetOptions::default()
            .conditional_set(ExistenceCheck::NX)
            .with_expiration(SetExpiry::EX(ttl_seconds))
            .get(true);

        let json = serde_json::to_string(&secret).context("secret deserialization error")?;

        let _: Option<String> = cnn.set_options(id.to_string(), json, opts)?;

        info!("stored secret entity: {}", secret);

        Ok(())
    }

    pub fn load(&self, id: &str) -> anyhow::Result<Option<Secret>> {
        info!("load secret by id '{id}'..");

        let client = redis::Client::open(&*self.cnn_url)?;
        let mut cnn = client.get_connection()?;

        let id = id.to_string();

        if cnn.exists(&id)? {
            let res: String = cnn.get(&id)?;

            let res = serde_json::from_str::<Secret>(&res);

            match res {
                Ok(secret) => {
                    if secret.download_policy == SecretDownloadPolicy::OneTime {
                        let _: String = cnn.del(&id)?;
                    }

                    info!("secret has been found");
                    Ok(Some(secret.clone()))
                }
                Err(e) => {
                    error!("{}", e);
                    Err(anyhow!("unable to get key"))
                }
            }
        } else {
            info!("secret wasn't found by id '{id}'");
            Ok(None)
        }
    }

    pub fn remove(&self, id: &str) -> anyhow::Result<()> {
        info!("remove secret by id '{id}'..");

        let client = redis::Client::open(&*self.cnn_url)?;
        let mut cnn = client.get_connection()?;

        let id = id.to_string();

        if cnn.exists(&id)? {
            let _: String = cnn.del(&id)?;

            info!("secret with id '{id}' has been removed");

            Ok(())
        } else {
            info!("secret wasn't found by id '{id}'");
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::secret::SecretDownloadPolicy;
    use crate::secret::storage::{DEFAULT_REDIS_CNN_URL, RedisSecretStorage};
    use crate::tests::secret::get_sample_secret;
    use crate::tests::string::get_random_string;

    #[ignore]
    #[test]
    fn secret_with_one_time_download_should_be_removed_after_load() {
        let storage = get_storage();

        let mut secret = get_sample_secret();
        secret.download_policy = SecretDownloadPolicy::OneTime;

        storage.store(&secret.id, &secret).unwrap();

        assert!(storage.load(&secret.id).unwrap().is_some());
        assert!(storage.load(&secret.id).unwrap().is_none());
    }

    #[ignore]
    #[test]
    fn secret_with_unlimited_time_download_should_not_be_removed_after_load() {
        let storage = get_storage();

        let mut secret = get_sample_secret();
        secret.download_policy = SecretDownloadPolicy::Unlimited;

        storage.store(&secret.id, &secret).unwrap();

        assert!(storage.load(&secret.id).unwrap().is_some());
        assert!(storage.load(&secret.id).unwrap().is_some());
        assert!(storage.load(&secret.id).unwrap().is_some());
        assert!(storage.load(&secret.id).unwrap().is_some());
        assert!(storage.load(&secret.id).unwrap().is_some());
    }

    #[ignore]
    #[test]
    fn remove_secret_test() {
        let storage = get_storage();

        let mut secret = get_sample_secret();
        secret.download_policy = SecretDownloadPolicy::Unlimited;

        storage.store(&secret.id, &secret).unwrap();
        storage.remove(&secret.id).unwrap();

        assert!(storage.load(&secret.id).unwrap().is_none());
    }

    #[ignore]
    #[test]
    fn return_none_for_unknown_secret() {
        let storage = get_storage();
        assert!(storage.load(&get_random_string()).unwrap().is_none());
    }

    fn get_storage() -> RedisSecretStorage {
        RedisSecretStorage::new(DEFAULT_REDIS_CNN_URL)
    }
}
