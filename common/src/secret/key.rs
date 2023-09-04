use uuid::Uuid;

/// Generate random encryption key
/// based on uuid
pub fn get_encryption_key() -> String {
    let uuid = Uuid::new_v4();
    uuid.to_string()
}

#[cfg(test)]
mod tests {
    use crate::secret::key::get_encryption_key;

    #[test]
    fn key_should_be_unique_each_time() {
        let key1 = get_encryption_key();
        let key2 = get_encryption_key();
        let key3 = get_encryption_key();

        assert_ne!(key1, key2);
        assert_ne!(key1, key3);
        assert_ne!(key2, key3);
    }
}