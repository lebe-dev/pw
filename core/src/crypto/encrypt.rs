use std::iter::repeat;
use std::str;

use crypto::aead::AeadEncryptor;
use crypto::aes_gcm::AesGcm;

use crate::crypto::get_valid_key;

/// Encrypt data with AES-GCM 256
/// Output is [hexNonce]/[hexCipher]/[hexMac] (nonce and iv are the same thing)
pub fn encrypt_data_with_aes256(data: &[u8], key: &str) -> String {
    let key_size = crypto::aes::KeySize::KeySize256;

    let valid_key = get_valid_key(key);

    let iv = get_iv(12);
    let mut cipher = AesGcm::new(key_size, &valid_key, &iv, &[]);

    let mut encrypted: Vec<u8> = repeat(0).take(data.len()).collect::<Vec<u8>>();
    let mut mac: Vec<u8> = repeat(0).take(16).collect();

    cipher.encrypt(data, &mut encrypted, &mut mac[..]);

    let hex_iv = hex::encode(iv);
    let hex_cipher = hex::encode(encrypted);
    let hex_mac = hex::encode(mac);
    let output = format!("{hex_iv}/{hex_cipher}/{hex_mac}");

    output
}

/// Creates an initial vector (iv). This is also called a nonce
fn get_iv(size: usize) -> Vec<u8> {
    let mut iv = vec![];
    for _ in 0..size {
        let r = rand::random();
        iv.push(r);
    }

    iv
}