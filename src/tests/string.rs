use fake::{Fake, Faker};

pub fn get_random_string() -> String {
    Faker.fake::<String>()
}
