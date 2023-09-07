use crate::secret::Secret;

pub trait SecretStorage {
    fn store(&self, id: &str, secret: &Secret);

    fn load(&self, id: &str) -> Option<Secret>;

    /// Remove expired entities
    fn cleanup(&self);
}