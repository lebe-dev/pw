use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,PartialEq,Clone,Debug)]
#[serde(rename_all = "kebab-case")]
pub struct Locale {
    pub id: String,

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
            id: "".to_string(),
            home_page: HomePageLabels {
                title: "".to_string(),
                message_placeholder: "".to_string(),
                secret_lifetime_title: "".to_string(),
                lifetime: LifetimeLabels {
                    one_hour: "".to_string(),
                    two_hours: "".to_string(),
                    one_day: "".to_string(),
                    one_time_download: "".to_string(),
                },
                encrypt_message_button: "".to_string(),
                secret_url_title: "".to_string(),
                copy_button: "".to_string(),
            },
            secret_url_page: SecretUrlPageLabels {
                title: "".to_string(),
            },
            secret_not_found_page: SecretNotFoundPageLabels {
                title: "".to_string(),
                possible_reasons_text: "".to_string(),
                possible_reasons_items: vec![],
            },
            footer_labels: FooterLabels {
                how_it_works: "".to_string(),
            },
        }
    }
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
}

#[derive(Serialize,Deserialize,PartialEq,Clone,Debug)]
#[serde(rename_all = "kebab-case")]
pub struct SecretUrlPageLabels {
    pub title: String
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