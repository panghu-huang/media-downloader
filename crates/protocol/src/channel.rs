use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadMediaRequest {
  pub channel: String,
  pub media_id: String,
  pub number: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadMediaResponse {
  pub destination_path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMediaMetadataRequest {
  pub channel: String,
  pub media_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaMetadata {
  pub channel: String,
  pub id: String,
  pub name: String,
  pub poster_url: String,
  pub release_year: u32,
  pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchMediaRequest {
  pub channel: Option<String>,
  pub keyword: String,
  pub page: u32,
  pub page_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchMediaResponse {
  pub items: Vec<MediaMetadata>,
  pub total: u32,
  pub page: u32,
  pub page_size: u32,
}

mod channel_inner {
  include!("./pb/channel.Channel.rs");
}

pub use channel_inner::{
  channel_client::ChannelClient,
  channel_server::{Channel as ChannelExt, ChannelServer},
};
