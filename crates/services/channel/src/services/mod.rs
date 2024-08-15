pub mod unified;
pub mod xiaobao;

use protocol::channel::TVShowMetadata;
use protocol::DownloadProgressReceiver;
use std::path::PathBuf;

pub struct DownloadTVShowOptions {
  pub tv_show_id: String,
  pub tv_show_episode_number: u32,
  pub destination_path: PathBuf,
}

#[async_trait::async_trait]
pub trait MediaChannelExt: Send + Sync {
  fn channel_name(&self) -> &'static str;
  async fn download_tv_show(
    &self,
    options: DownloadTVShowOptions,
  ) -> anyhow::Result<DownloadProgressReceiver>;
  async fn get_tv_show_metadata(&self, tv_show_id: &str) -> anyhow::Result<TVShowMetadata>;
}
