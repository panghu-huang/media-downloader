pub mod unified;

use protocol::channel::MediaMetadata;
use protocol::DownloadProgressReceiver;
use std::path::PathBuf;

pub struct DownloadMediaOptions {
  pub media_id: String,
  pub number: Option<u32>,
  pub destination_path: PathBuf,
}

#[async_trait::async_trait]
pub trait MediaChannelExt: Send + Sync {
  fn channel_name(&self) -> &'static str;
  async fn download_media(
    &self,
    options: DownloadMediaOptions,
  ) -> anyhow::Result<DownloadProgressReceiver>;
  async fn get_media_metadata(&self, media_id: &str) -> anyhow::Result<MediaMetadata>;
}
