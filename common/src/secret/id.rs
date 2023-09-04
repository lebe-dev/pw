use uuid::Uuid;

pub fn get_secret_id() -> String {
    let uuid = Uuid::new_v4();
    uuid.to_string()
}

#[cfg(test)]
mod tests {
    use crate::secret::id::get_secret_id;

    #[test]
    fn id_should_be_unique_each_time() {
        let id1 = get_secret_id();
        let id2 = get_secret_id();
        let id3 = get_secret_id();

        assert_ne!(id1, id2);
        assert_ne!(id1, id3);
        assert_ne!(id2, id3);
    }
}