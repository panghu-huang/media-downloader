use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeItem {
  pub type_id: u32,
  pub type_pid: u32,
  pub type_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListItem {
  #[serde(rename = "vod_id")]
  pub id: u32,
  #[serde(rename = "vod_name")]
  pub name: String,
  pub type_id: u32,
  pub type_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Detail {
  #[serde(rename = "vod_id")]
  pub id: u32,
  #[serde(rename = "vod_name")]
  pub name: String,
  pub type_id: u32,
  pub type_name: String,
  #[serde(rename = "vod_year")]
  pub year: String,
  #[serde(rename = "vod_pic")]
  pub picture: String,
  #[serde(rename = "vod_content")]
  pub description: String,
  #[serde(rename = "vod_play_url")]
  pub play_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response<T> {
  pub code: u32,
  pub msg: String,
  pub page: u32,
  #[serde(rename = "pagecount")]
  pub page_count: u32,
  pub total: u32,
  pub limit: String,
  pub list: Vec<T>,
  pub class: Option<Vec<TypeItem>>,
}

pub type ListResponse = Response<ListItem>;

pub type DetailResponse = Response<Detail>;

pub struct UnifiedAPI {
  base_url: String,
}

impl UnifiedAPI {
  pub async fn get_details(&self, ids: &[&str]) -> anyhow::Result<DetailResponse> {
    let client = Client::new();

    let res = client
      .get(&self.base_url)
      .query(&[("ac", "detail"), ("ids", &ids.join(","))])
      .send()
      .await
      .map_err(|e| anyhow::anyhow!("Failed to send request: {}", e))?;

    let status = res.status();

    if !status.is_success() {
      anyhow::bail!("Request failed with status: {}", status);
    }

    let text = res
      .text()
      .await
      .map_err(|e| anyhow::anyhow!("Failed to get text response: {}", e))?;

    let res: DetailResponse = serde_json::from_str(&text)
      .map_err(|e| anyhow::anyhow!("Failed to decode json response: {} ({})", e, text))?;

    if res.code != 1 {
      anyhow::bail!("Request failed with response: {:#?}", res);
    }

    log::info!("Successful got response of ids '{:?}'", ids);

    Ok(res)
  }
}

impl UnifiedAPI {
  pub fn new(base_url: &str) -> Self {
    Self {
      base_url: base_url.to_owned(),
    }
  }
}
