use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadTVShowRequest {
  pub channel: String,
  pub tv_show_id: String,
  pub tv_show_season_number: u32,
  pub tv_show_episode_number: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadTVShowResponse {
  pub destination_path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTVShowMetadataRequest {
  pub channel: String,
  pub tv_show_id: String,
  pub tv_show_season_number: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TVShowMetadata {
  pub channel: String,
  pub id: String,
  pub name: String,
  pub year: u32,
  pub season_number: u32,
  pub total_episodes: u32,
  pub source_page_url: String,
  pub source_download_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DownloadProgress {
  Started {
    total_segments_of_meida: usize,
    started_at: String,
  },
  InProgress {
    message: String,
    started_at: String,
  },
  Done {
    completed_at: String,
  },
}

mod channel_inner {
  include!("./pb/channel.Channel.rs");
}

pub use channel_inner::{
  channel_client::ChannelClient,
  channel_server::{Channel as ChannelExt, ChannelServer},
};
