use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringOrNumber {
  String(String),
  Number(u32),
}

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
pub struct ListRequest {
  pub page: u32,
  pub keyword: Option<String>,
  pub type_id: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response<T> {
  pub code: u32,
  pub msg: String,
  pub page: StringOrNumber,
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
  http_version: http::Version,
}

const REQUEST_USER_AGENT: &str = 
      "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/127.0.0.0 Safari/537.36";

impl UnifiedAPI {
  pub async fn list(&self, request: &ListRequest) -> anyhow::Result<ListResponse> {
    let client = self.request_client()?;

    let query = &[
      ("ac", "list"),
      ("pg", &request.page.to_string()),
      ("wd", &request.keyword.clone().unwrap_or_default()),
      ("t", &request.type_id.unwrap_or_default().to_string()),
    ];

    let res = client
      .get(&self.base_url)
      .query(query)
      .header("User-Agent", REQUEST_USER_AGENT)
      .version(self.http_version)
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

    let res: ListResponse = serde_json::from_str(&text)
      .map_err(|e| anyhow::anyhow!("Failed to decode json response: {} ({})", e, text))?;

    if res.code != 1 {
      anyhow::bail!("Request failed with response: {:#?}", res);
    }

    log::info!("Successful got response of page {}", request.page);

    Ok(res)
  }

  pub async fn get_details<T: AsRef<str>>(&self, ids: &[T]) -> anyhow::Result<DetailResponse> {
    let client = self.request_client()?;

    let ids = ids.iter().map(|s| s.as_ref()).collect::<Vec<_>>().join(",");
    log::info!("Getting details with ids: {}", ids);

    let res = client
      .get(&self.base_url)
      .query(&[("ac", "detail"), ("ids", &ids)])
      .header("User-Agent", REQUEST_USER_AGENT)
      .version(self.http_version)
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

    Ok(res)
  }

  fn request_client(&self) -> anyhow::Result<Client> {
    if self.http_version == http::Version::HTTP_2 {
      Client::builder()
      .http2_prior_knowledge()
        .use_rustls_tls()
        .build()
        .map_err(|e| anyhow::anyhow!("Failed to create new request client: {}", e))
    } else {
      Ok(Client::new())
    }
  }
}

impl UnifiedAPI {
  pub fn new(base_url: &str) -> Self {
    Self {
      base_url: base_url.to_owned(),
      http_version: http::Version::HTTP_2,
    }
  }

  pub fn new_with_http_1(base_url: &str) -> Self {
    Self {
      base_url: base_url.to_owned(),
      http_version: http::Version::HTTP_11
    }
  }
}

impl From<StringOrNumber> for u32 {
  fn from(val: StringOrNumber) -> Self {
    match val {
      StringOrNumber::Number(value) => {
        value
      }
      StringOrNumber::String(value) => {
        value.parse().unwrap()
      }
    }
  }
}
