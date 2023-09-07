use log::error;

use crate::error::OperationError;

pub fn get_encoded_url_slug(secret_id: &str, encryption_key: &str, additional_data: &str) -> String {
    let data = format!("{secret_id}|{encryption_key}|{additional_data}");
    base64::encode(data)
}

pub fn get_encoded_url_slug_parts(data: &str) -> Result<(String,String,String), OperationError> {
    match base64::decode(data) {
        Ok(decoded) => {
            match String::from_utf8(decoded) {
                Ok(decoded_str) => {
                    let parts = decoded_str.split("|").collect::<Vec<&str>>();

                    if parts.len() == 3 {
                        let secret_id = parts.first().unwrap();
                        let private_key = parts.get(1).unwrap();
                        let additional_data_hex = parts.last().unwrap();
                        Ok((secret_id.to_string(), private_key.to_string(), additional_data_hex.to_string()))

                    } else {
                        error!("decoded payload contains unexpected amount of parts: {}", parts.len());
                        Err(OperationError::UrlDecodeError)
                    }
                }
                Err(e) => {
                    error!("{}", e);
                    Err(OperationError::UrlDecodeError)
                }
            }
        }
        Err(e) => {
            error!("base64 decode error: {}", e);
            Err(OperationError::UrlDecodeError)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::error::OperationError;
    use crate::secret::url::{get_encoded_url_slug, get_encoded_url_slug_parts};

    #[test]
    fn return_url_slug_parts() {
        let secret_id = "abcdef";
        let private_key = "fj209fj039fjsd";
        let additional_data = get_random_additional_data().unwrap();
        let additional_data_hex = hex::encode(additional_data);

        let encoded_url_slug = get_encoded_url_slug(
            &secret_id, &private_key, &additional_data_hex);

        let result = get_encoded_url_slug_parts(&encoded_url_slug).unwrap();

        assert_eq!(secret_id, result.0);
        assert_eq!(private_key, result.1);
        assert_eq!(additional_data_hex, result.2);
    }

    #[test]
    fn return_error_for_less_encoded_parts() {
        let encoded_url_slug = base64::encode("2039jf293jf");

        match get_encoded_url_slug_parts(&encoded_url_slug) {
            Err(e) => {
                match e {
                    OperationError::UrlDecodeError => assert!(true),
                    _ => panic!("OperationError::UrlDecodeError expected")
                }
            }
            Ok(_) => panic!("error expected")
        }
    }

    #[test]
    fn return_error_for_invalid_encoded_url_slug() {
        match get_encoded_url_slug_parts("INVALID-VALUE") {
            Err(e) => {
                match e {
                    OperationError::UrlDecodeError => assert!(true),
                    _ => panic!("OperationError::UrlDecodeError expected")
                }
            }
            Ok(_) => panic!("error expected")
        }
    }

    fn get_random_additional_data() -> Result<[u8; 15], getrandom::Error> {
        let mut buf = [0u8; 15];
        getrandom::getrandom(&mut buf)?;
        Ok(buf)
    }
}