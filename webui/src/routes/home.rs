use aes_wasm::aes256gcm::*;
use dioxus::prelude::*;

const KEY_LENGTH: usize = 32;

pub fn get_valid_key(key: &str) -> [u8; 32] {
    let mut bytes = key.as_bytes().to_vec();

    if bytes.len() < KEY_LENGTH {
        for _ in 0..(KEY_LENGTH - bytes.len()) {
            bytes.push(0x00);
        }

    } else if bytes.len() > KEY_LENGTH {
        bytes = bytes[0..KEY_LENGTH].to_vec();
    }

    bytes.try_into().unwrap()
}

pub fn HomePage(cx: Scope) -> Element {

    let key = get_valid_key("testDemo");
    let nonce = Nonce::default();
    let msg = b"hello world";
    let ad = b"additional data";
    let (ciphertext, tag) = encrypt_detached(msg, ad, &key, nonce);
    let plaintext = decrypt_detached(ciphertext, &tag, ad, &key, nonce).unwrap();
    let ciphertext_and_tag = encrypt(msg, ad, &key, nonce);

    let encrypted = hex::encode(&ciphertext_and_tag);

    let plaintext = decrypt(ciphertext_and_tag, ad, &key, nonce).unwrap();

    let decrypted = String::from_utf8_lossy(&plaintext).to_string();

    cx.render(rsx! {
        div {
            class: "mb-3",
            div {
                format!("PW APP :: UNDER DEVELOPMENT"),
            },
            div {
                format!("encrypted: {encrypted}")
            },
            div {
                format!("decrypted: {decrypted}")
            }
        }
    })
}