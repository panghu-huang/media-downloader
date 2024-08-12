use crate::common::EasySelector;
use crate::common::{download_video, DownloadVideoOptions};
use crate::services::DownloadTVShowOptions;
use crate::services::MediaChannelExt;
use protocol::channel::TVShowMetadata;
use regex::Regex;
use reqwest::Client;
use scraper::{Html, Selector};

const REQUEST_USER_AGENT: &str = 
      "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/127.0.0.0 Safari/537.36";

pub struct SimpleTVShowMetadata {
  name: String,
  year: u32,
  total_episodes: u32,
}

pub trait SimpleTVShowMetadataExtractor {
  fn extract_tv_show_metadata(&self) -> anyhow::Result<SimpleTVShowMetadata>;
}

pub struct XiaobaoTV {
  host: String,
}

#[async_trait::async_trait]
impl MediaChannelExt for XiaobaoTV {
  fn channel_name(&self) -> &'static str {
    "xiaobao"
  }

  async fn download_tv_show(&self, options: DownloadTVShowOptions) -> anyhow::Result<()> {
    let html = self
      .fetch_tv_show_web_page_html(
        &options.tv_show_id,
        options.tv_show_season_number,
        options.tv_show_episode_number,
      )
      .await?;

    let download_url = self.extract_download_url_from_web_page_html(&html).await?;

    download_video(DownloadVideoOptions {
      download_url: &download_url,
      destination_path: &options.destination_path,
      parallel_size: 10,
    })
    .await?;

    Ok(())
  }

  async fn get_tv_show_metadata(
    &self,
    tv_show_id: &str,
    tv_show_season_number: u32,
  ) -> anyhow::Result<TVShowMetadata> {
    let web_page_url = self.tv_show_web_page_url(tv_show_id, tv_show_season_number, 1);

    let html = self.fetch_web_page_html(&web_page_url).await?;

    let download_url = self.extract_download_url_from_web_page_html(&html).await?;

    let simple_metadata = self.extract_tv_show_metadata_from_web_page(&html).await?;

    Ok(TVShowMetadata {
      channel: self.channel_name().to_owned(),
      id: tv_show_id.to_owned(),
      name: simple_metadata.name,
      year: simple_metadata.year,
      season_number: tv_show_season_number,
      source_page_url: web_page_url,
      source_download_url: download_url,
      total_episodes: simple_metadata.total_episodes,
    })
  }
}

impl XiaobaoTV {
  async fn extract_download_url_from_web_page_html(
    &self,
    web_page_html: &str,
  ) -> anyhow::Result<String> {
    let reg = Regex::new(r#"","url":"((\S)+)","url_next"#)?;

    let caps = reg.captures(web_page_html);

    Ok(caps.unwrap().get(1).unwrap().as_str().replace("\\/", "/"))
  }

  async fn extract_tv_show_metadata_from_web_page(
    &self,
    tv_show_web_page_html: &str,
  ) -> anyhow::Result<SimpleTVShowMetadata> {
    Html::parse_document(tv_show_web_page_html).extract_tv_show_metadata()
  }

  async fn fetch_tv_show_web_page_html(
    &self,
    tv_show_id: &str,
    tv_show_season_number: u32,
    tv_show_episode_number: u32,
  ) -> anyhow::Result<String> {
    let web_page_url =
      self.tv_show_web_page_url(tv_show_id, tv_show_season_number, tv_show_episode_number);

    self.fetch_web_page_html(&web_page_url).await
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
        REQUEST_USER_AGENT,
      )
      .version(http::Version::HTTP_2)
      .send()
      .await?
      .text()
      .await?;

    Ok(html)
  }

  fn tv_show_web_page_url(
    &self,
    tv_show_id: &str,
    tv_show_season_number: u32,
    tv_show_episode_number: u32,
  ) -> String {
    format!(
      "https://{}/index.php/vod/play/id/{}/sid/{}/nid/{}.html",
      self.host, tv_show_id, tv_show_season_number, tv_show_episode_number,
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

impl SimpleTVShowMetadataExtractor for Html {
  fn extract_tv_show_metadata(&self) -> anyhow::Result<SimpleTVShowMetadata> {
    let title_selector = Selector::parse(".title.text-fff").unwrap();

    let title_element = self.select(&title_selector).next().unwrap();

    let year_selector = Selector::parse(".myui-player__data > .text-muted > .text-muted").unwrap();

    let year_element = self.select(&year_selector).last().unwrap();

    Ok(SimpleTVShowMetadata {
      name: title_element.inner_text(),
      year: year_element.inner_text().parse()?,
      // TODO: implement this
      total_episodes: 0,
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn test_get_tv_show_metadata() {
    let tv_show_id = 548.to_string();
    let tv_show_season_number = 1;

    let xiaobaotv = XiaobaoTV::new("xiaoxintv.com");

    let metadata = xiaobaotv
      .get_tv_show_metadata(&tv_show_id, tv_show_season_number)
      .await
      .unwrap();

    assert_eq!(metadata.name, "海贼王");
    assert_eq!(metadata.year, 1999);
  }

  #[tokio::test]
  async fn test_download_tv_show() {
    let tv_show_id = 548.to_string();
    let tv_show_season_number = 1;
    let tv_show_episode_number = 1;

    let xiaobaotv = XiaobaoTV::new("xiaoxintv.com");

    let destination_path = std::env::current_dir()
      .unwrap()
      .join("downloads")
      .join("test.mp4");

    let download_opts = DownloadTVShowOptions {
      tv_show_id: tv_show_id.clone(),
      tv_show_season_number,
      tv_show_episode_number,
      destination_path,
    };

    xiaobaotv.download_tv_show(download_opts).await.unwrap();
  }
}
