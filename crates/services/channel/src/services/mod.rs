pub mod xiaobao;

use protocol::channel::DownloadTVShowResponse;
use std::path::PathBuf;

pub struct DownloadTVShowOptions {
  pub tv_show_id: String,
  pub tv_show_season_number: u32,
  pub tv_show_episode_number: u32,
  pub destination_dir: PathBuf,
}

#[async_trait::async_trait]
pub trait MediaChannelExt: Send + Sync {
  async fn download_tv_show(
    &self,
    options: DownloadTVShowOptions,
  ) -> anyhow::Result<DownloadTVShowResponse>;
}
