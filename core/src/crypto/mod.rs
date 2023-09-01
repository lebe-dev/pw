pub mod decrypt;
pub mod encrypt;

/// Gets a valid key. This must be exactly 16 bytes. if less than 16 bytes, it will be padded with 0.
/// If more than 16 bytes, it will be truncated
pub fn get_valid_key(key: &str) -> Vec<u8> {
    let mut bytes = key.as_bytes().to_vec();

    if bytes.len() < 16 {
        for _ in 0..(16 - bytes.len()) {
            bytes.push(0x00);
        }

    } else if bytes.len() > 16 {
        bytes = bytes[0..16].to_vec();
    }

    bytes
}

#[cfg(test)]
mod tests {
    use std::str::from_utf8;

    use log::info;

    use crate::crypto::decrypt::decrypt;
    use crate::crypto::encrypt::encrypt;
    use crate::tests::init_logging;

    #[test]
    fn test_encrypt_decrypt() {
        init_logging();

        let data = "VERY IMPORTANT data HERE";
        let password = "SuPerHardC0ReSeC";

        let res = encrypt(data.as_bytes(), password);

        info!("encrypted: '{res}'");

        let decrypted_bytes = decrypt(res.as_str(), password).unwrap();
        let decrypted_string = from_utf8(&decrypted_bytes).unwrap();

        assert_eq!(decrypted_string, data);
    }
}