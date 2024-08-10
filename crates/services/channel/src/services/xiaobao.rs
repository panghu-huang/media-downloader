use crate::common::EasySelector;
use crate::services::MediaChannelExt;
use crate::services::{DownloadTVShowOptions, DownloadedTVShow};
use reqwest::Client;
use scraper::{Html, Selector};

pub struct TVShowDetails {
  name: String,
  year: u32,
}

pub struct XiaobaoTV {
  host: String,
}

impl MediaChannelExt for XiaobaoTV {
  async fn download_tv_show(options: DownloadTVShowOptions) -> anyhow::Result<DownloadedTVShow> {
    todo!()
  }
}

impl XiaobaoTV {
  async fn extract_tv_show_details_from_web_page(
    &self,
    tv_show_id: String,
    tv_season_number: u32,
    tv_episode_number: u32,
  ) -> anyhow::Result<TVShowDetails> {
    let tv_show_web_page = self.tv_show_web_page(tv_show_id, tv_season_number, tv_episode_number);

    let html = self.fetch_web_page_html(&tv_show_web_page).await?;

    Self::parse_details_from_html(&html)
  }

  fn parse_details_from_html(html: &str) -> anyhow::Result<TVShowDetails> {
    let document = Html::parse_document(html);

    let title_selector = Selector::parse(".title.text-fff").unwrap();

    let title_element = document.select(&title_selector).last().unwrap();

    let year_selector = Selector::parse(".myui-player__data > .text-muted > .text-muted").unwrap();

    let year_element = document.select(&year_selector).last().unwrap();

    Ok(TVShowDetails {
      name: title_element.inner_text(),
      year: year_element.inner_text().parse()?,
    })
  }

  async fn fetch_web_page_html(&self, web_page_url: &str) -> anyhow::Result<String> {
    let client = Client::builder()
      .http2_adaptive_window(true)
      .http2_prior_knowledge()
      .use_rustls_tls()
      .build()?;

    let html = client.get(web_page_url)
    .header(
      "user-agent", 
      "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/127.0.0.0 Safari/537.36"
    )
    .version(http::Version::HTTP_2)
    .send()
    .await?
    .text()
    .await?;

    Ok(html)
  }

  fn tv_show_web_page(
    &self,
    tv_show_id: String,
    tv_season_number: u32,
    tv_episode_number: u32,
  ) -> String {
    format!(
      "https://{}/index.php/vod/play/id/{}/sid/{}/nid/{}.html",
      self.host, tv_show_id, tv_season_number, tv_episode_number,
    )
  }
}

impl XiaobaoTV {
  pub fn new(host: &str) -> Self {
    Self {
      host: host.to_owned(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn test_extract_tv_show_details_from_web_page() {
    let tv_show_id = 548;
    let tv_show_season_number = 1;
    let tv_show_episode_number = 1;

    let xiaobaotv = XiaobaoTV::new("xiaoxintv.com");

    let tv_show_details = xiaobaotv
      .extract_tv_show_details_from_web_page(
        tv_show_id.to_string(),
        tv_show_season_number,
        tv_show_episode_number,
      )
      .await
      .unwrap();

    assert_eq!(tv_show_details.year, 1999);
    assert_eq!(tv_show_details.name, "海贼王");
  }
}
