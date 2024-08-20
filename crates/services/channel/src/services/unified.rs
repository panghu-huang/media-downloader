mod api;

use self::api::{Detail, UnifiedAPI};
use crate::services::DownloadMediaOptions;
use crate::services::MediaChannelExt;
use protocol::channel::MediaMetadata;
use protocol::channel::{SearchMediaRequest, SearchMediaResponse};
use protocol::DownloadProgressReceiver;

pub struct UnifiedMediaService {
  api: UnifiedAPI,
  channel_name: String,
}

#[async_trait::async_trait]
impl MediaChannelExt for UnifiedMediaService {
  fn channel_name(&self) -> &'static str {
    Box::leak(Box::new(self.channel_name.clone()))
  }

  async fn download_media(
    &self,
    options: DownloadMediaOptions,
  ) -> anyhow::Result<DownloadProgressReceiver> {
    let detail = self.get_media_detail(&options.media_id).await?;

    let mut play_url_list = detail.play_url.split('#');

    let number: usize = options.number.unwrap_or(1).try_into()?;

    let url_of_number = play_url_list.nth(number - 1).ok_or_else(|| {
      anyhow::anyhow!(
        "Invalid number {:?} of media {}",
        options.number,
        options.media_id
      )
    })?;

    let m3u8_url = url_of_number.split('$').nth(1).ok_or_else(|| {
      anyhow::anyhow!(
        "Invalid url format found: {} ({}#{:?})",
        url_of_number,
        options.media_id,
        options.number,
      )
    })?;

    let download_opts = crate::common::DownloadMediaOptions {
      download_url: m3u8_url,
      destination_path: &options.destination_path,
      parallel_size: 10,
    };

    let progress = crate::common::download_media(download_opts).await?;

    Ok(progress)
  }

  async fn get_media_metadata(&self, media_id: &str) -> anyhow::Result<MediaMetadata> {
    let detail = self.get_media_detail(media_id).await?;

    let release_year = detail.year.parse().unwrap_or(0);

    Ok(MediaMetadata {
      channel: self.channel_name.clone(),
      id: detail.id.to_string(),
      name: detail.name,
      release_year,
      description: detail.description,
    })
  }

  async fn search_media(
    &self,
    request: &SearchMediaRequest,
  ) -> anyhow::Result<SearchMediaResponse> {
    let search_result = self.api.search(request.keyword.as_str()).await?;

    let items = search_result
      .list
      .iter()
      .map(|item| protocol::channel::MediaMetadata {
        channel: self.channel_name.clone(),
        id: item.id.to_string(),
        name: item.name.clone(),
        release_year: item.year.parse().unwrap_or(0),
        description: item.description.clone(),
      })
      .collect();

    Ok(SearchMediaResponse { items })
  }
}

impl UnifiedMediaService {
  async fn get_media_detail(&self, id: &str) -> anyhow::Result<Detail> {
    log::info!("Getting video detail of {}", id);

    let details = self.api.get_details(&[id]).await?;

    if details.list.is_empty() {
      anyhow::bail!("Invalid TV show ID: {}", id);
    }

    let detail = details.list.first().unwrap();

    Ok(detail.clone())
  }
}

impl UnifiedMediaService {
  pub fn new(channel_name: &str, base_url: &str) -> Self {
    let api = UnifiedAPI::new(base_url);

    Self {
      channel_name: channel_name.to_owned(),
      api,
    }
  }
}
