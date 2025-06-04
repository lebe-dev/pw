use fake::{Fake, Faker};

use crate::secret::model::{
    Secret, SecretContentType, SecretDownloadPolicy, SecretFileMetadata, SecretTTL,
};

use super::string::get_random_string;

pub fn get_sample_secret() -> Secret {
    Secret {
        id: get_random_string(),
        payload: Faker.fake::<String>(),
        ttl: SecretTTL::OneHour,
        download_policy: SecretDownloadPolicy::Unlimited,
        content_type: SecretContentType::Text,
        metadata: SecretFileMetadata {
            name: get_random_string(),
            r#type: "text".to_string(),
            size: 0,
        },
    }
}
