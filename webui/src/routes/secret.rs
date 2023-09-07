use aes_wasm::aes256gcm::{decrypt, Nonce};
use dioxus::prelude::*;
use log::{error, info};

use common::dto::AppConfigDto;
use common::locale::Locale;
use common::secret::{Secret, SecretDownloadPolicy};
use common::secret::url::get_encoded_url_slug_parts;

use crate::components::footer::PageFooter;
use crate::components::header::PageHeader;
use crate::config::fetch_app_config;
use crate::routes::home::get_valid_key;
use crate::secret::get_secret_by_id;

#[inline_props]
pub fn SecretPage(cx: Scope, encoded_id: String) -> Element {
    info!("secret id: {encoded_id}");

    let force_get_app_config_dto = use_state(cx, || ());

    let app_config_state = use_state::<AppConfigDto>(cx, || AppConfigDto {
        message_max_length: 4096,
        locale: Locale::default()
    });

    let (secret_id, private_key) = get_encoded_url_slug_parts(&encoded_id)
        .unwrap_or(("invalid-slug".to_string(), "".to_string()));

    info!("secret id '{secret_id}'");

    let force_get_secret = use_state(cx, || ());

    let secret_state = use_state::<Option<Secret>>(cx, || None);

    {
        let app_config = app_config_state.clone();

        use_effect(cx, force_get_app_config_dto, |_| async move {
            match fetch_app_config().await {
                Ok(config) => {
                    info!("config: {:?}", config);
                    app_config.set(config);
                }
                Err(e) => error!("unable to fetch app config: {}", e)
            }

        });
    }

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
            class: "container bg-white p-3 p-lg-5 text-center shadow-sm",

            div {
                class: "mb-4 text-start",

                match secret_state.get() {
                  Some(secret) => {
                    let payload = hex::decode(&secret.payload).expect("unable to decode hex");
                    info!("payload decode - ok");

                    // TODO: replace with random
                    let ad: &[u8; 15] = b"SuPpErStr0Ng038";

                    let key = get_valid_key(&private_key);
                    let nonce = Nonce::default();

                    let message = decrypt(payload, ad, &key, nonce).unwrap();

                    info!("decrypt - ok");

                    let message = String::from_utf8_lossy(&message).to_string();

                    rsx! {
                        div {
                            class: "text-start mb-3",
                            h5 {
                                "{app_config_state.locale.secret_url_page.title}"
                            }
                        },
                        div {
                            id: "message",
                            class: "p-3 rounded-2 bg-light",
                            "{message}"
                        },
                        if secret.download_policy == SecretDownloadPolicy::OneTime {
                            rsx! {
                                div {
                                    class: "p-1 rounded-3 border border-warning text-dark mt-3 mb-3",
                                    "{app_config_state.locale.secret_url_page.one_time_download_precaution_message}"
                                }
                            }
                        },
                    }
                  }
                  None => {
                    rsx! {
                        div {
                            class: "text-start mb-3",
                            h5 {
                                "{app_config_state.locale.secret_not_found_page.title}"
                            }
                        },
                        div {
                            class: "mb-2",
                            "{app_config_state.locale.secret_not_found_page.possible_reasons_text}:"
                        },

                        ul {
                            class: "mb-5",
                            app_config_state.locale.secret_not_found_page.possible_reasons_items.iter().map(|reason| {
                                rsx! {
                                    li {
                                        "{reason}"
                                    },
                                }
                            })
                        }

                    }
                  }
                },
            },

            PageFooter {
                how_it_works_label: "{app_config_state.locale.footer_labels.how_it_works}",
                locale_id: "{app_config_state.locale.id}"
            }
        },
      }
    })
}