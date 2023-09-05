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