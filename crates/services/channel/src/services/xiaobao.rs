use crate::common::EasySelector;
use crate::common::{download_video, DownloadVideoOptions};
use crate::services::DownloadTVShowOptions;
use crate::services::MediaChannelExt;
use protocol::channel::DownloadTVShowResponse;
use regex::Regex;
use reqwest::Client;
use scraper::{Html, Selector};

pub struct MediaDetails {
  name: String,
  year: u32,
}

pub trait MediaDetailsParser {
  fn parse_media_details(&self) -> anyhow::Result<MediaDetails>;
}

pub struct XiaobaoTV {
  host: String,
}

#[async_trait::async_trait]
impl MediaChannelExt for XiaobaoTV {
  async fn download_tv_show(
    &self,
    options: DownloadTVShowOptions,
  ) -> anyhow::Result<DownloadTVShowResponse> {
    let html = self
      .fetch_tv_show_web_page_html(
        &options.tv_show_id,
        options.tv_show_season_number,
        options.tv_show_episode_number,
      )
      .await?;

    let download_url = self.extract_download_url_from_web_page_html(&html).await?;

    let file_name = format!(
      "{}-{}-{}.mp4",
      options.tv_show_id, options.tv_show_season_number, options.tv_show_episode_number
    );

    let destination_path = options.destination_dir.join(file_name);

    download_video(DownloadVideoOptions {
      download_url: &download_url,
      destination_path: &destination_path,
      parallel_size: 10,
    })
    .await?;

    let details = self.extract_media_details_from_web_page(&html).await?;

    Ok(DownloadTVShowResponse {
      tv_show_id: options.tv_show_id,
      tv_show_season_number: options.tv_show_season_number,
      tv_show_episode_number: options.tv_show_episode_number,
      tv_show_name: details.name,
      tv_show_year: details.year,
      destination_path,
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

  async fn extract_media_details_from_web_page(
    &self,
    tv_show_web_page_html: &str,
  ) -> anyhow::Result<MediaDetails> {
    Html::parse_document(tv_show_web_page_html).parse_media_details()
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
      "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/127.0.0.0 Safari/537.36"
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

impl MediaDetailsParser for Html {
  fn parse_media_details(&self) -> anyhow::Result<MediaDetails> {
    let title_selector = Selector::parse(".title.text-fff").unwrap();

    let title_element = self.select(&title_selector).next().unwrap();

    let year_selector = Selector::parse(".myui-player__data > .text-muted > .text-muted").unwrap();

    let year_element = self.select(&year_selector).last().unwrap();

    Ok(MediaDetails {
      name: title_element.inner_text(),
      year: year_element.inner_text().parse()?,
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn test_extract_tv_show_details_from_web_page() {
    let tv_show_id = 548.to_string();
    let tv_show_season_number = 1;
    let tv_show_episode_number = 1;

    let xiaobaotv = XiaobaoTV::new("xiaoxintv.com");

    let html = xiaobaotv
      .fetch_tv_show_web_page_html(&tv_show_id, tv_show_season_number, tv_show_episode_number)
      .await
      .unwrap();

    let tv_show_details = xiaobaotv
      .extract_media_details_from_web_page(&html)
      .await
      .unwrap();

    assert_eq!(tv_show_details.name, "海贼王");
    assert_eq!(tv_show_details.year, 1999);
  }

  #[tokio::test]
  async fn test_download_tv_show() {
    let tv_show_id = 548.to_string();
    let tv_show_season_number = 1;
    let tv_show_episode_number = 1;

    let xiaobaotv = XiaobaoTV::new("xiaoxintv.com");

    let destination_dir = std::env::current_dir().unwrap().join("downloads");

    let download_opts = DownloadTVShowOptions {
      tv_show_id: tv_show_id.clone(),
      tv_show_season_number,
      tv_show_episode_number,
      destination_dir,
    };

    let res = xiaobaotv.download_tv_show(download_opts).await.unwrap();

    assert_eq!(res.tv_show_id, tv_show_id);
  }
}
