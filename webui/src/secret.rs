use anyhow::anyhow;
use log::info;
use reqwest::StatusCode;

use common::secret::Secret;

use crate::config::get_base_host;

pub async fn store_secret(secret: &Secret) -> anyhow::Result<()> {
    info!("store secret: {}", secret);

    let builder = reqwest::ClientBuilder::new();
    let client = builder.build()?;

    let url = format!("{}/api/secret", get_base_host());

    let resp = client.post(url).json(&secret).send().await?;

    info!("response: {:?}", resp);

    let status = resp.status();

    if status == StatusCode::OK {
        Ok(())

    } else {
        Err(anyhow!("unable to store secret"))
    }
}

pub async fn get_secret_by_id(secret_id: &str) -> anyhow::Result<Option<Secret>> {
    info!("fetching secret by id '{secret_id}'..");

    let url = format!("{}/api/secret/{secret_id}", get_base_host());

    let resp = reqwest::get(url).await?;

    let status = resp.status();

    if status == StatusCode::OK {
        let secret = resp.json::<Secret>().await?;

        info!("secret received: {:?}", secret);

        Ok(Some(secret))

    } else {
        Ok(None)
    }
}