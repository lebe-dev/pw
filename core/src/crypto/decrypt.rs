use std::error::Error;
use std::io;
use std::io::ErrorKind;
use std::iter::repeat;
use std::str::from_utf8;

use crypto::aead::AeadDecryptor;
use crypto::aes_gcm::AesGcm;

use crate::crypto::get_valid_key;

///Decryption using AES-GCM 128
///iv_data_mac is a string that contains the iv/nonce, data, and mac values. All these values
/// must be hex encoded, and separated by "/" i.e. [hex(iv)/hex(data)/hex(mac)]. This function decodes
/// the values. key (or password) is the raw (not hex encoded) password
pub fn decrypt(iv_data_mac: &str, key: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let (iv, data, mac) = split_iv_data_mac(iv_data_mac)?;
    let key = get_valid_key(key);

    let key_size = crypto::aes::KeySize::KeySize128;

    let mut decipher = AesGcm::new(key_size, &key, &iv, &[]);

    let mut dst: Vec<u8> = repeat(0).take(data.len()).collect();
    let result = decipher.decrypt(&data, &mut dst, &mac);

    if result { println!("Successful decryption"); }
    println!("\nDecrypted {}", from_utf8(&dst).unwrap());

    Ok(dst)
}

/// orig must be a string of the form [hexNonce]/[hexCipherText]/[hexMac]. This
/// is the data returned from encrypt(). This function splits the data, removes
/// the hex encoding, and returns each as a list of bytes.
fn split_iv_data_mac(orig: &str) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>), Box<dyn Error>> {
    let split: Vec<&str> = orig.split('/').into_iter().collect();

    if split.len() != 3 {
        return Err(Box::new(io::Error::from(ErrorKind::Other)));
    }
    let iv_res = hex::decode(split[0]);
    if iv_res.is_err() {
        return Err(Box::new(io::Error::from(ErrorKind::Other)));
    }

    let iv = iv_res.unwrap();

    let data_res = hex::decode(split[1]);
    if data_res.is_err() {
        return Err(Box::new(io::Error::from(ErrorKind::Other)));
    }
    let data = data_res.unwrap();

    let mac_res = hex::decode(split[2]);
    if mac_res.is_err() {
        return Err(Box::new(io::Error::from(ErrorKind::Other)));
    }
    let mac = mac_res.unwrap();

    Ok((iv, data, mac))
}