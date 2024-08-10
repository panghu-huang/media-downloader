pub mod xiaobao;

use std::path::PathBuf;

pub struct DownloadTVShowOptions {
  tv_show_id: String,
  tv_sesson_number: u64,
  tv_episode_number: u64,
  destination_dir: PathBuf,
}

pub struct DownloadedTVShow {
  tv_show_id: String,
  tv_sesson_number: u64,
  tv_episode_number: u64,
  tv_show_name: String,
  tv_show_year: u32,
  destination_path: PathBuf,
}

pub trait MediaChannelExt {
  async fn download_tv_show(options: DownloadTVShowOptions) -> anyhow::Result<DownloadedTVShow>;
}
