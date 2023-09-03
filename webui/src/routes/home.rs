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
            nav {
                class: "navbar bg-dark text-info",
                div {
                    class: "container-fluid",
                    a {
                        class: "navbar-brand text-light",
                        href: "#",
                        "PW"
                    }
                }
            },
            div {
                class: "container bg-white p-5 text-center shadow-sm",
                div {
                    class: "text-start",
                    h4 {
                        "Message"
                    }
                }
                textarea {
                    id: "message-input",
                    class: "form-control",
                    rows: 5,
                    autofocus: true,
                    placeholder: "The data will be encrypted in the browser"
                },
                div {
                    div {
                        class: "mt-3",
                        "Secret lifetime:"
                    },
                    label {
                        id: "ttl-one-hour",
                        class: "mt-2 me-2",
                        input {
                            id: "ttl-one-hour",
                            name: "secret-ttl",
                            r#type: "radio",
                            class: "me-1",
                            checked: true
                        },
                        "One hour"
                    },
                    label {
                        id: "ttl-two-hours",
                        class: "me-2",
                        input {
                            id: "ttl-two-hours",
                            name: "secret-ttl",
                            r#type: "radio",
                            class: "me-1"
                        },
                        "Two hours"
                    },
                    label {
                        id: "ttl-one-day",
                        class: "me-2",
                        input {
                            id: "ttl-one-day",
                            name: "secret-ttl",
                            r#type: "radio",
                            class: "me-1"
                        },
                        "One day"
                    }
                },

                div {
                  class: "mt-3",
                  label {
                        id: "one-time-download",
                        class: "mt-2 me-2",
                        input {
                            id: "one-time-download",
                            name: "one-time-download",
                            r#type: "checkbox",
                            class: "me-1"
                        },
                        "One time download"
                    }
                },

                button {
                    id: "encrypt-btn",
                    class: "btn btn-dark mt-5",
                    disabled: true,
                    "Encrypt message"
                },

                div {
                    class: "footer-links mt-5",
                    span {
                        class: "me-1",
                        "v1.0.0"
                    },
                    span {
                        class: "ms-1 me-1",
                        "|"
                    },
                    a {
                        class: "me-1 ms-1",
                        "HOW IT WORKS"
                    }
                    span {
                        class: "ms-1 me-1",
                        "|"
                    },
                    a {
                        class: "ms-1",
                        "GITHUB"
                    }
                }
            }
        }
    })
}