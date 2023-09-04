use log::info;
use web_sys::window;

use common::dto::AppConfigDto;

pub async fn fetch_app_config() -> anyhow::Result<AppConfigDto> {
    info!("fetching app config..");

    let url = format!("{}/api/config", get_base_host());

    let resp = reqwest::get(url).await?;

    let config = resp.json::<AppConfigDto>().await?;

    info!("config received: {:?}", config);

    Ok(config)
}

pub fn get_base_host() -> String {
    let protocol = window().unwrap().location().protocol().unwrap();
    let hostname = window().unwrap().location().hostname().unwrap();
    let port = window().unwrap().location().port().unwrap();

    if !port.is_empty() && port != "80" && port != "443" {
        format!("{protocol}//{hostname}:{port}")

    } else {
        format!("{protocol}//{hostname}")
    }
}