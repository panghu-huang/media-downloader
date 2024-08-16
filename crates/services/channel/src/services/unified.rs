mod api;

use self::api::{Detail, UnifiedAPI};
use crate::common::{download_video, DownloadVideoOptions};
use crate::services::DownloadTVShowOptions;
use crate::services::MediaChannelExt;
use protocol::channel::TVShowMetadata;
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

  async fn download_tv_show(
    &self,
    options: DownloadTVShowOptions,
  ) -> anyhow::Result<DownloadProgressReceiver> {
    let detail = self.get_video_detail(&options.tv_show_id).await?;

    let mut play_url_list = detail.play_url.split('#');

    let episode: usize = options.tv_show_episode_number.try_into()?;

    let url_of_episode = play_url_list.nth(episode - 1).ok_or_else(|| {
      anyhow::anyhow!(
        "Invalid episode number {} of TV show {}",
        options.tv_show_episode_number,
        options.tv_show_id
      )
    })?;

    let m3u8_url = url_of_episode.split('$').nth(1).ok_or_else(|| {
      anyhow::anyhow!(
        "Invalid url format found: {} ({}#{})",
        url_of_episode,
        options.tv_show_id,
        options.tv_show_episode_number
      )
    })?;

    let progress = download_video(DownloadVideoOptions {
      download_url: m3u8_url,
      destination_path: &options.destination_path,
      parallel_size: 10,
    })
    .await?;

    Ok(progress)
  }

  async fn get_tv_show_metadata(&self, tv_show_id: &str) -> anyhow::Result<TVShowMetadata> {
    let detail = self.get_video_detail(tv_show_id).await?;

    let total_episodes = detail.play_url.split('#').count();

    let year = detail.year.parse().unwrap_or(0);

    Ok(TVShowMetadata {
      channel: self.channel_name.clone(),
      id: detail.id.to_string(),
      name: detail.name,
      year,
      total_episodes: total_episodes.try_into()?,
      description: detail.description,
    })
  }
}

impl UnifiedMediaService {
  async fn get_video_detail(&self, id: &str) -> anyhow::Result<Detail> {
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
