use aes_wasm::aes256gcm::{decrypt, Nonce};
use dioxus::prelude::*;
use log::{error, info};

use common::secret::Secret;
use common::secret::url::get_encoded_url_slug_parts;

use crate::components::footer::PageFooter;
use crate::components::header::PageHeader;
use crate::routes::home::get_valid_key;
use crate::secret::get_secret_by_id;

#[inline_props]
pub fn SecretPage(cx: Scope, encoded_id: String) -> Element {
    info!("secret id: {encoded_id}");

    let (secret_id, private_key) = get_encoded_url_slug_parts(&encoded_id)
        .unwrap_or(("invalid-slug".to_string(), "".to_string()));

    info!("secret id '{secret_id}'");

    let force_get_secret = use_state(cx, || ());

    let secret_state = use_state::<Option<Secret>>(cx, || None);

    {
        let secret_state = secret_state.clone();

        use_effect(cx, force_get_secret, |_| async move {
            match get_secret_by_id(&secret_id).await {
                Ok(secret) => {
                    secret_state.set(secret)
                }
                Err(e) => error!("unable to fetch app config: {}", e)
            }

        });
    }

    cx.render(rsx! {
      div {
        PageHeader {},

        div {
            class: "container bg-white p-5 text-center shadow-sm",

            div {
                class: "mb-4 text-start",

                match secret_state.get() {
                  Some(secret) => {
                    info!("RENDER EXISTING SECRET: {}", secret);
                    let payload = hex::decode(&secret.payload).expect("unable to decode hex");
                    info!("payload decode - ok");

                    // TODO: replace with random
                    let ad: &[u8; 15] = b"additional data";

                    let key = get_valid_key(&private_key);
                    let nonce = Nonce::default();

                    let message = decrypt(payload, ad, &key, nonce).unwrap();

                    info!("decrypt - ok");

                    let message = String::from_utf8_lossy(&message).to_string();

                    rsx! {
                        div {
                            class: "text-start mb-3",
                            h5 {
                                "Message"
                            }
                        },
                        div {
                            id: "message",
                            class: "p-3 rounded-2 bg-light",
                            "{message}"
                        }
                    }
                  }
                  None => {
                    rsx! {
                        div {
                            class: "text-start mb-3",
                            h5 {
                                "Secret wasn't found"
                            }
                        },
                        div {
                          "Possible reasons:"
                        },
                        ul {
                            class: "mb-5",
                            li {
                                "Link has been expired"
                            },
                            li {
                                "It was one-time link and someone opened it already"
                            }
                        }
                    }
                  }
                },
            },

            PageFooter {}
        },
      }
    })
}