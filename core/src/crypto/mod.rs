pub mod decrypt;
pub mod encrypt;

/// Gets a valid key. This must be exactly 32 bytes. if less than 32 bytes, it will be padded with 0.
/// If more than 32 bytes, it will be truncated
pub fn get_valid_key(key: &str) -> Vec<u8> {
    let mut bytes = key.as_bytes().to_vec();

    if bytes.len() < 32 {
        for _ in 0..(32 - bytes.len()) {
            bytes.push(0x00);
        }

    } else if bytes.len() > 32 {
        bytes = bytes[0..32].to_vec();
    }

    bytes
}

#[cfg(test)]
mod tests {
    use std::str::from_utf8;

    use crate::crypto::decrypt::decrypt_aes256_data;
    use crate::crypto::encrypt::encrypt_data_with_aes256;
    use crate::tests::get_random_string;

    #[test]
    fn key_greater_than_32_chars_must_be_supported() {
        assert_data_decrypted(
    &get_random_string(),
        "12345678901234567890123456789012345678901234567890"
        );
    }

    #[test]
    fn key_length_less_than_32_must_be_supported() {
        assert_data_decrypted(&get_random_string(), "abc");
    }

    fn assert_data_decrypted(input_data: &str, key: &str) {
        let encrypted_string = encrypt_data_with_aes256(input_data.as_bytes(), key);
        let decrypted_bytes = decrypt_aes256_data(encrypted_string.as_str(), key).unwrap();
        let decrypted_string = from_utf8(&decrypted_bytes).unwrap();
        assert_eq!(input_data, decrypted_string);
    }
}