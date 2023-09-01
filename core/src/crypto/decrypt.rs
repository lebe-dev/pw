use std::error::Error;
use std::io;
use std::io::ErrorKind;
use std::iter::repeat;

use crypto::aead::AeadDecryptor;
use crypto::aes_gcm::AesGcm;
use log::error;

use crate::crypto::get_valid_key;
use crate::error::OperationError;

/// Decryption using AES-GCM 256
/// `iv_data_mac` is a string that contains the `iv/nonce`, `data`, and `mac` values. All these values
/// must be hex encoded, and separated by "/" i.e. [hex(iv)/hex(data)/hex(mac)]. This function decodes
/// the values. key (or password) is the raw (not hex encoded) password
pub fn decrypt_aes256_data(iv_data_mac: &str, key: &str) -> Result<Vec<u8>, OperationError> {
    match split_iv_data_mac(iv_data_mac) {
        Ok((iv, data, mac)) => {
            let key = get_valid_key(key);

            let key_size = crypto::aes::KeySize::KeySize256;

            let mut decipher = AesGcm::new(key_size, &key, &iv, &[]);

            let mut dst: Vec<u8> = repeat(0).take(data.len()).collect();
            let result = decipher.decrypt(&data, &mut dst, &mac);

            if result {
                Ok(dst)

            } else {
                Err(OperationError::DecryptionError)
            }
        }

        Err(e) => {
            error!("decryption error: {}", e);
            Err(OperationError::DecryptionError)
        }
    }
}

/// orig must be a string of the form [hexNonce]/[hexCipherText]/[hexMac]. This
/// is the data returned from encrypt(). This function splits the data, removes
/// the hex encoding, and returns each as a list of bytes.
fn split_iv_data_mac(orig: &str) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>), Box<dyn Error>> {
    let split: Vec<&str> = orig.split('/').into_iter().collect();

    if split.len() != 3 {
        error!("invalid input string than expected: [hexNonce]/[hexCipherText]/[hexMac]");
        return Err(Box::new(io::Error::from(ErrorKind::Other)));
    }

    match hex::decode(split[0]) {
        Ok(iv) => {

            match hex::decode(split[1]) {
                Ok(data) => {

                    match hex::decode(split[2]) {
                        Ok(mac) => Ok((iv, data, mac)),
                        Err(e) => {
                            error!("hex decode error: {}", e);
                            Err(Box::new(io::Error::from(ErrorKind::Other)))
                        }
                    }

                }
                Err(e) => {
                    error!("hex decode error: {}", e);
                    Err(Box::new(io::Error::from(ErrorKind::Other)))
                }
            }

        }
        Err(e) => {
            error!("hex decode error: {}", e);
            Err(Box::new(io::Error::from(ErrorKind::Other)))
        }
    }
}