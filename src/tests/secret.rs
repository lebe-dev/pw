use fake::{Fake, Faker};

use crate::secret::model::{Secret, SecretDownloadPolicy, SecretTTL};

use super::string::get_random_string;

pub fn get_sample_secret() -> Secret {
    Secret {
        id: get_random_string(),
        payload: Faker.fake::<String>(),
        ttl: SecretTTL::OneHour,
        download_policy: SecretDownloadPolicy::Unlimited,
    }
}
