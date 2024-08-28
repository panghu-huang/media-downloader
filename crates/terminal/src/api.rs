use protocol::media::SearchMediaResponse;

#[derive(Clone)]
pub struct MediaAPI {
  base_url: String,
}

#[derive(Clone)]
pub struct SearchMediaOptions {
  pub keyword: String,
  pub page: u32,
}

impl MediaAPI {
  pub fn new(base_url: &str) -> Self {
    Self {
      base_url: base_url.to_string(),
    }
  }

  pub async fn search(&self, options: &SearchMediaOptions) -> anyhow::Result<SearchMediaResponse> {
    let url = format!(
      "{}/api/v1/media/search?keyword={}&page={}",
      self.base_url, options.keyword, options.page
    );

    let response = reqwest::Client::new()
      .get(&url)
      .send()
      .await?
      .json::<SearchMediaResponse>()
      .await?;

    Ok(response)
  }
}

#[derive(Clone)]
#[allow(clippy::upper_case_acronyms)]
pub struct API {
  pub media: MediaAPI,
}

impl API {
  pub fn new(base_url: &str) -> Self {
    Self {
      media: MediaAPI::new(base_url),
    }
  }
}
