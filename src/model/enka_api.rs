use crate::entity::Resbody;
use log::info;
use reqwest;

pub async fn connect_api(uuid: &str) -> Result<Resbody, Box<dyn std::error::Error>> {
    info!("{}のプレイデータを取得します", uuid);
    let url = "https://enka.network/api/uid/".to_string() + uuid;

    info!("{}", url);

    let res = reqwest::Client::new()
        .get(url)
        .header(reqwest::header::USER_AGENT, "crewzca_practice/1.0")
        .send()
        .await?;

    info!("{:?}", res);

    let text = res.text().await?;

    let inf = serde_json::from_str::<Resbody>(&text);

    return Ok(inf?);
}
