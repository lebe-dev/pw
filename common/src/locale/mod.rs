use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,PartialEq,Clone,Debug)]
#[serde(rename_all = "camelCase")]
pub struct Locale {
    pub id: String,

    pub messages: MessageLabels,

    pub errors: ErrorLabels,

    #[serde(alias = "home-page")]
    pub home_page: HomePageLabels,
    #[serde(alias = "secret-url-page")]
    pub secret_url_page: SecretUrlPageLabels,
    #[serde(alias = "secret-not-found-page")]
    pub secret_not_found_page: SecretNotFoundPageLabels,
    #[serde(alias = "footer")]
    pub footer_labels: FooterLabels
}

impl Default for Locale {
    fn default() -> Self {
        Locale {
            id: "en".to_string(),
            messages: MessageLabels {
                loading_title: "Loading..".to_string(),
                error_title: "Error".to_string(),
            },
            errors: ErrorLabels {
                loading_data: "Couldn't load data".to_string(),
                store_secret: "Store secret error".to_string(),
            },
            home_page: HomePageLabels {
                title: "Message".to_string(),
                message_placeholder: "The data will be encrypted in the browser".to_string(),
                secret_lifetime_title: "Secret lifetime".to_string(),
                lifetime: LifetimeLabels {
                    one_hour: "One hour".to_string(),
                    two_hours: "Two hours".to_string(),
                    one_day: "One day".to_string(),
                    one_time_download: "One time download".to_string(),
                    one_time_download_precaution_message: "This link is for one-time use only, so don't try to open it or the secret will disappear.".to_string(),
                },
                encrypt_message_button: "Encrypt message".to_string(),
                secret_url_title: "Secret URL".to_string(),
                copy_button: "Copy".to_string(),
            },
            secret_url_page: SecretUrlPageLabels {
                title: "Message".to_string(),
                one_time_download_precaution_message: "This link is for one-time use only, so don't try to open it or the secret will disappear.".to_string(),
            },
            secret_not_found_page: SecretNotFoundPageLabels {
                title: "Secret wasn't found".to_string(),
                possible_reasons_text: "Possible reasons".to_string(),
                possible_reasons_items: vec![
                    "Link has been expired".to_string(),
                    "It was one-time link and someone opened it already".to_string()
                ],
            },
            footer_labels: FooterLabels {
                how_it_works: "FAQ".to_string(),
            },
        }
    }
}

#[derive(Serialize,Deserialize,PartialEq,Clone,Debug)]
#[serde(rename_all = "camelCase")]
pub struct MessageLabels {
    #[serde(alias = "loading-title")]
    pub loading_title: String,

    #[serde(alias = "error-title")]
    pub error_title: String
}

#[derive(Serialize,Deserialize,PartialEq,Clone,Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorLabels {
    #[serde(alias = "loading-data")]
    pub loading_data: String,

    #[serde(alias = "store-secret")]
    pub store_secret: String
}

#[derive(Serialize,Deserialize,PartialEq,Clone,Debug)]
#[serde(rename_all = "camelCase")]
pub struct HomePageLabels {
    pub title: String,
    #[serde(alias = "message-placeholder")]
    pub message_placeholder: String,
    #[serde(alias = "secret-lifetime-title")]
    pub secret_lifetime_title: String,
    pub lifetime: LifetimeLabels,
    #[serde(alias = "encrypt-message-button")]
    pub encrypt_message_button: String,
    #[serde(alias = "secret-url-title")]
    pub secret_url_title: String,
    #[serde(alias = "copy-button")]
    pub copy_button: String
}

#[derive(Serialize,Deserialize,PartialEq,Clone,Debug)]
#[serde(rename_all = "camelCase")]
pub struct LifetimeLabels {
    #[serde(alias = "one-hour")]
    pub one_hour: String,
    #[serde(alias = "two-hours")]
    pub two_hours: String,
    #[serde(alias = "one-day")]
    pub one_day: String,
    #[serde(alias = "one-time-download")]
    pub one_time_download: String,
    #[serde(alias = "one-time-download-precaution-message")]
    pub one_time_download_precaution_message: String,
}

#[derive(Serialize,Deserialize,PartialEq,Clone,Debug)]
#[serde(rename_all = "camelCase")]
pub struct SecretUrlPageLabels {
    pub title: String,

    #[serde(alias = "one-time-download-precaution-message")]
    pub one_time_download_precaution_message: String,
}

#[derive(Serialize,Deserialize,PartialEq,Clone,Debug)]
#[serde(rename_all = "camelCase")]
pub struct SecretNotFoundPageLabels {
    pub title: String,
    #[serde(alias = "possible-reasons-text")]
    pub possible_reasons_text: String,
    #[serde(alias = "possible-reasons-items")]
    pub possible_reasons_items: Vec<String>
}

#[derive(Serialize,Deserialize,PartialEq,Clone,Debug)]
#[serde(rename_all = "camelCase")]
pub struct FooterLabels {
    #[serde(alias = "how-it-works")]
    pub how_it_works: String
}