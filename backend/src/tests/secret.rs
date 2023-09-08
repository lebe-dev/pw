use common::secret::{Secret, SecretDownloadPolicy, SecretTTL};
use common::tests::get_random_string;

pub fn get_sample_secret() -> Secret {
    Secret {
        id: get_random_string(),
        payload: get_random_string(),
        ttl: SecretTTL::OneHour,
        download_policy: SecretDownloadPolicy::Unlimited
    }
}