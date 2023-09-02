use std::fmt::{Display, Formatter};

use chrono::Local;
use log::info;
use mini_moka::sync::Cache;

use core::secret::Secret;
use core::secret::SecretDownloadPolicy;
use core::secret::SecretTTL;
use core::secret::storage::SecretStorage;

pub struct InMemorySecretStorage {
    cache: Cache<String,SecretEntity>
}

#[derive(Clone)]
pub struct SecretEntity {
    pub secret: Secret,

    /// Secret creation (unix)time, in seconds
    pub created: i64,
}

impl Display for SecretEntity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[SecretEntity] secret: {}, created: {} [/SecretEntity]", self.secret, self.created)
    }
}

impl SecretEntity {
    fn is_expired(&self) -> bool {
        let now_unixtime = Local::now().timestamp();

        let minutes_passed = (now_unixtime - self.created) / 60;

        let expired: bool;

        match self.secret.ttl {
            SecretTTL::OneHour => expired = minutes_passed > 60,
            SecretTTL::TwoHours => expired = minutes_passed > 60 * 2,
            SecretTTL::OneDay => expired = minutes_passed > 60 * 24
        }

        info!("secret '{}' expired: {expired}", self.secret.id);

        expired
    }
}

impl InMemorySecretStorage {
    pub fn new(cache: Cache<String,SecretEntity>) -> InMemorySecretStorage {
        InMemorySecretStorage {
            cache
        }
    }
}

impl SecretStorage for InMemorySecretStorage {
    fn store(&self, id: &str, secret: &Secret) {
        let secret_entity = SecretEntity {
            secret: secret.clone(),
            created: Local::now().timestamp(),
        };

        self.cache.insert(id.to_string(), secret_entity.clone());

        info!("stored secret entity: {}", secret_entity);
    }

    fn load(&self, id: &str) -> Option<Secret> {
        info!("load secret by id '{id}'..");
        match self.cache.get(&id.to_string()) {
            Some(secret) => {
                if secret.secret.download_policy == SecretDownloadPolicy::OneTime {
                    self.cache.invalidate(&id.to_string());
                }

                let expired = secret.is_expired();

                for secret_entity in self.cache.iter() {
                    if secret_entity.is_expired() {
                        self.cache.invalidate(&secret_entity.secret.id.to_string());
                        info!("secret (id '{}') has been removed", secret_entity.secret.id)
                    }
                }

                if !expired {
                    Some(secret.secret.clone())

                } else {
                    None
                }
            }
            None => {
                info!("secret wasn't found by id '{id}'");
                None
            }
        }
    }
}

#[cfg(test)]
mod is_expired_tests {
    use chrono::{Duration, Local};

    use core::secret::SecretTTL;

    use crate::secret::storage::SecretEntity;
    use crate::tests::secret::get_sample_secret;

    #[test]
    fn one_hour_expiration() {
        assert_non_expired_entity(SecretTTL::OneHour, 0);
        assert_non_expired_entity(SecretTTL::OneHour, 15);
        assert_non_expired_entity(SecretTTL::OneHour, 59);

        assert_expired_entity(SecretTTL::OneHour, 61);
        assert_expired_entity(SecretTTL::OneHour, 127);
        assert_expired_entity(SecretTTL::OneHour, 119);
    }

    #[test]
    fn two_hours_expiration() {
        assert_non_expired_entity(SecretTTL::TwoHours, 0);
        assert_non_expired_entity(SecretTTL::TwoHours, 15);
        assert_non_expired_entity(SecretTTL::TwoHours, 119);

        assert_expired_entity(SecretTTL::TwoHours, 121);
        assert_expired_entity(SecretTTL::TwoHours, 127);
        assert_expired_entity(SecretTTL::TwoHours, 999);
    }

    #[test]
    fn one_day_expiration() {
        assert_non_expired_entity(SecretTTL::OneDay, 0);
        assert_non_expired_entity(SecretTTL::OneDay, 70);
        assert_non_expired_entity(SecretTTL::OneDay, 1300);

        assert_expired_entity(SecretTTL::OneDay, 1441);
        assert_expired_entity(SecretTTL::OneDay, 999999);
    }

    fn assert_non_expired_entity(ttl: SecretTTL, minutes_from_now: i64) {
        let entity = get_secret_entity(ttl, minutes_from_now);
        assert!(!entity.is_expired())
    }

    fn assert_expired_entity(ttl: SecretTTL, minutes_from_now: i64) {
        let entity = get_secret_entity(ttl, minutes_from_now);
        assert!(entity.is_expired())
    }

    fn get_secret_entity(ttl: SecretTTL, minutes_from_now: i64) -> SecretEntity {
        let mut secret = get_sample_secret();
        secret.ttl = ttl;

        SecretEntity {
            secret,
            created: get_created_time(minutes_from_now),
        }
    }

    fn get_created_time(minutes_from_now: i64) -> i64 {
        Local::now()
            .checked_sub_signed(Duration::minutes(minutes_from_now)).unwrap()
            .timestamp()
    }
}

#[cfg(test)]
mod tests {
    use mini_moka::sync::Cache;

    use core::secret::SecretDownloadPolicy;
    use core::secret::SecretTTL;
    use core::secret::storage::SecretStorage;

    use crate::secret::storage::{InMemorySecretStorage, SecretEntity};
    use crate::tests::get_random_string;
    use crate::tests::secret::get_sample_expired_secret_entity;
    use crate::tests::secret::get_sample_secret;

    #[test]
    fn expired_secrets_should_be_removed_during_any_load_attempt() {
        let cache: Cache<String, SecretEntity> = Cache::new(1000);

        let expired_secret_entity1 = get_sample_expired_secret_entity();
        let expired_secret_entity2 = get_sample_expired_secret_entity();

        cache.insert(
            expired_secret_entity1.secret.id.to_string(), expired_secret_entity1.clone());
        cache.insert(
            expired_secret_entity2.secret.id.to_string(), expired_secret_entity2.clone());

        let storage = InMemorySecretStorage::new(cache);

        let mut non_expired_secret = get_sample_secret();
        non_expired_secret.download_policy = SecretDownloadPolicy::Unlimited;
        non_expired_secret.ttl = SecretTTL::OneDay;

        storage.store(&non_expired_secret.id, &non_expired_secret.clone());

        assert!(storage.load(&expired_secret_entity1.secret.id).is_none());
        assert!(storage.load(&expired_secret_entity2.secret.id).is_none());
        assert!(storage.load(&non_expired_secret.id).is_some());
    }

    #[test]
    fn secret_with_existing_id_must_be_overwritten() {
        let storage = get_storage();

        let secret1 = get_sample_secret();
        storage.store(&secret1.id, &secret1);

        let mut secret2 = get_sample_secret();
        secret2.id = secret1.id.clone();

        storage.store(&secret2.id, &secret2);

        let result = storage.load(&secret1.id).unwrap();

        assert_eq!(secret2, result);
    }

    #[test]
    fn secret_with_one_time_download_should_be_removed_after_load() {
        let storage = get_storage();

        let mut secret = get_sample_secret();
        secret.download_policy = SecretDownloadPolicy::OneTime;

        storage.store(&secret.id, &secret);

        assert!(storage.load(&secret.id).is_some());
        assert!(storage.load(&secret.id).is_none());
    }

    #[test]
    fn secret_with_unlimited_time_download_should_not_be_removed_after_load() {
        let storage = get_storage();

        let mut secret = get_sample_secret();
        secret.download_policy = SecretDownloadPolicy::Unlimited;

        storage.store(&secret.id, &secret);

        assert!(storage.load(&secret.id).is_some());
        assert!(storage.load(&secret.id).is_some());
        assert!(storage.load(&secret.id).is_some());
        assert!(storage.load(&secret.id).is_some());
        assert!(storage.load(&secret.id).is_some());
    }

    #[test]
    fn return_none_for_unknown_secret() {
        let storage = get_storage();
        assert!(storage.load(&get_random_string()).is_none());
    }

    fn get_storage() -> impl SecretStorage {
        let cache = Cache::new(1000);
        InMemorySecretStorage::new(cache)
    }


}