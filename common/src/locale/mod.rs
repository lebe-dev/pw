use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,PartialEq,Clone,Debug)]
#[serde(rename_all = "kebab-case")]
pub struct Locale {
    pub id: String,

    pub messages: MessageLabels,

    pub errors: ErrorLabels,

    #[serde(alias = "homePage")]
    pub home_page: HomePageLabels,
    #[serde(alias = "secretUrlPage")]
    pub secret_url_page: SecretUrlPageLabels,
    #[serde(alias = "secretNotFoundPage")]
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
#[serde(rename_all = "kebab-case")]
pub struct MessageLabels {
    #[serde(alias = "loadingTitle")]
    pub loading_title: String,

    #[serde(alias = "errorTitle")]
    pub error_title: String
}

#[derive(Serialize,Deserialize,PartialEq,Clone,Debug)]
#[serde(rename_all = "kebab-case")]
pub struct ErrorLabels {
    #[serde(alias = "loadingData")]
    pub loading_data: String,

    #[serde(alias = "storeSecret")]
    pub store_secret: String
}

#[derive(Serialize,Deserialize,PartialEq,Clone,Debug)]
#[serde(rename_all = "kebab-case")]
pub struct HomePageLabels {
    pub title: String,
    #[serde(alias = "messagePlaceholder")]
    pub message_placeholder: String,
    #[serde(alias = "secretLifetimeTitle")]
    pub secret_lifetime_title: String,
    pub lifetime: LifetimeLabels,
    #[serde(alias = "encryptMessageButton")]
    pub encrypt_message_button: String,
    #[serde(alias = "secretUrlTitle")]
    pub secret_url_title: String,
    #[serde(alias = "copyButton")]
    pub copy_button: String
}

#[derive(Serialize,Deserialize,PartialEq,Clone,Debug)]
#[serde(rename_all = "kebab-case")]
pub struct LifetimeLabels {
    #[serde(alias = "oneHour")]
    pub one_hour: String,
    #[serde(alias = "twoHours")]
    pub two_hours: String,
    #[serde(alias = "oneDay")]
    pub one_day: String,
    #[serde(alias = "oneTimeDownload")]
    pub one_time_download: String,
    #[serde(alias = "oneTimeDownloadPrecautionMessage")]
    pub one_time_download_precaution_message: String,
}

#[derive(Serialize,Deserialize,PartialEq,Clone,Debug)]
#[serde(rename_all = "kebab-case")]
pub struct SecretUrlPageLabels {
    pub title: String,

    #[serde(alias = "oneTimeDownloadPrecautionMessage")]
    pub one_time_download_precaution_message: String,
}

#[derive(Serialize,Deserialize,PartialEq,Clone,Debug)]
#[serde(rename_all = "kebab-case")]
pub struct SecretNotFoundPageLabels {
    pub title: String,
    #[serde(alias = "possibleReasonsText")]
    pub possible_reasons_text: String,
    #[serde(alias = "possibleReasonsItems")]
    pub possible_reasons_items: Vec<String>
}

#[derive(Serialize,Deserialize,PartialEq,Clone,Debug)]
#[serde(rename_all = "kebab-case")]
pub struct FooterLabels {
    #[serde(alias = "howItWorks")]
    pub how_it_works: String
}