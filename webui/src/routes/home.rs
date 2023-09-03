use dioxus::prelude::*;
use log::{error, info};

use common::dto::AppConfigDto;
use common::secret::SecretTTL;

use crate::config::fetch_app_config;

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
                        href: "#",
                        "HOW IT WORKS"
                    }
                    span {
                        class: "ms-1 me-1",
                        "|"
                    },
                    a {
                        class: "ms-1",
                        href: "#",
                        "GITHUB"
                    }
                }
            }
        }
    })
}