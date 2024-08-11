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
  pub tv_show_id: String,
  pub tv_show_season_number: u32,
  pub tv_show_episode_number: u32,
  pub tv_show_name: String,
  pub tv_show_year: u32,
  pub destination_path: PathBuf,
}

mod channel_inner {
  include!("./pb/channel.Channel.rs");
}

pub use channel_inner::{
  channel_client::ChannelClient,
  channel_server::{Channel as ChannelExt, ChannelServer},
};
