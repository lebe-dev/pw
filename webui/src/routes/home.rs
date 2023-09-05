use aes_wasm::aes256gcm::{encrypt, Nonce};
use dioxus::prelude::*;
use log::{error, info};

use common::dto::AppConfigDto;
use common::secret::{Secret, SecretDownloadPolicy, SecretTTL};
use common::secret::id::get_secret_id;
use common::secret::key::get_encryption_key;
use common::secret::url::get_encoded_url_slug;

use crate::components::footer::PageFooter;
use crate::components::header::PageHeader;
use crate::config::{fetch_app_config, get_base_host};
use crate::secret::store_secret;

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
    let force_get_app_config_dto = use_state(cx, || ());

    let app_config_state = use_state::<AppConfigDto>(cx, || AppConfigDto {
        message_max_length: 4096,
    });

    let message_max_length_state = use_state::<u16>(cx, || 0);
    let message_length_state = use_state::<u16>(cx, || 0);

    let is_form_valid_state = use_state::<bool>(cx, || false);
    let message_state = use_state::<String>(cx, || "".to_string());

    let secret_ttl_state = use_state::<SecretTTL>(cx, || SecretTTL::OneHour);
    let one_time_download_state = use_state::<bool>(cx, || false);

    let secret_url_state = use_state::<Option<String>>(cx, || None);

    {
        let app_config = app_config_state.clone();
        let message_max_length_state = message_max_length_state.clone();
        use_effect(cx, force_get_app_config_dto, |_| async move {
            match fetch_app_config().await {
                Ok(config) => {
                    info!("config: {:?}", config);
                    message_max_length_state.set(config.message_max_length);
                    app_config.set(config);
                }
                Err(e) => error!("unable to fetch app config: {}", e)
            }

        });
    }

    let on_encrypt_message = move |_| {

        cx.spawn({

            let message_state = message_state.clone();
            let secret_ttl_state = secret_ttl_state.clone();
            let one_time_download_state = one_time_download_state.clone();
            let secret_url_state = secret_url_state.clone();

            let encryption_key = &get_encryption_key();
            let encryption_key_array = get_valid_key(&encryption_key);
            let nonce = Nonce::default();

            // TODO: replace with random
            let ad: &[u8; 15] = b"additional data";

            let ciphertext = encrypt(
                message_state.get(), ad, &encryption_key_array, nonce);

            let payload = hex::encode(ciphertext);

            let download_policy: SecretDownloadPolicy = if *one_time_download_state.get() {
                SecretDownloadPolicy::OneTime

            } else {
                SecretDownloadPolicy::Unlimited
            };

            let secret = Secret {
                id: get_secret_id(),
                payload: payload.to_string(),
                ttl: secret_ttl_state.get().clone(),
                download_policy
            };

            let url_slug_for_encode = get_encoded_url_slug(&secret.id, encryption_key);

            let url = format!("{}/secret/{}", get_base_host(), url_slug_for_encode);

            async move {
                store_secret(&secret).await.unwrap();
                secret_url_state.set(Some(url));
            }

        });

    };

    let content = match secret_url_state.get() {
        Some(url) => {
            let eval = use_eval(cx).clone();
            eval("new ClipboardJS('#copy-url-button');").expect("ClipboardJS initialization error");

            rsx! {
                div {
                    class: "text-start",
                    h5 {
                        "Secret URL"
                    }
                },
                div {
                    id: "url",
                    class: "secret-url p-3 rounded-3 bg-light mb-3",
                    format!("{url}")
                },
                button {
                    id: "copy-url-button",
                    class: "btn btn-sm btn-dark",
                    "data-clipboard-target": "#url",
                    title: "Copy to clipboard",
                    "Copy"
                }
            }
        }
        None => {
            rsx! {
                div {
                    class: "text-start",
                    h5 {
                        "Message"
                    }
                },
                textarea {
                    id: "message-input",
                    class: "form-control mb-1",
                    rows: 5,
                    autofocus: true,
                    placeholder: "The data will be encrypted in the browser",
                    maxlength: "{message_max_length_state}",
                    oninput: move |evt| {
                        let value = evt.value.clone();
                        info!("message: {value}");
                        message_length_state.set(value.len() as u16);

                        message_state.set(value.to_string());

                        if value.is_empty() {
                            is_form_valid_state.set(false);

                        } else {
                            is_form_valid_state.set(true);
                        }
                    }
                },
                div {
                    id: "message-length-usage",
                    class: "font-monospace",
                    "{message_length_state} / {message_max_length_state}"
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
                            class: "prevent-select me-1",
                            checked: true,
                            onclick: move |_| {
                                secret_ttl_state.set(SecretTTL::OneHour)
                            }
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
                            class: "prevent-select me-1",
                            onclick: move |_| {
                                secret_ttl_state.set(SecretTTL::TwoHours)
                            }
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
                            class: "prevent-select me-1",
                            onclick: move |_| {
                                secret_ttl_state.set(SecretTTL::OneDay)
                            }
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
                            class: "prevent-select me-1",
                            oninput: move |evt| {
                                let value = evt.value.clone();
                                info!("value: {value}");

                                match value.as_str() {
                                    "true" => one_time_download_state.set(true),
                                    _ => one_time_download_state.set(false)
                                };
                            }
                        },
                        "One time download"
                    }
                },

                button {
                    id: "encrypt-btn",
                    r#type: "button",
                    class: "btn btn-dark mt-5",
                    disabled: "{!is_form_valid_state}",
                    onclick: on_encrypt_message,
                    "Encrypt message"
                },
            }
        }
    };

    cx.render(rsx! {
        div {
            PageHeader {},
            div {
                class: "container bg-white p-5 text-center shadow-sm",

                content,

                PageFooter {}
            }
        }
    })
}