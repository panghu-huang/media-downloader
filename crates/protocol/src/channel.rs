use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChannelsRequest {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelInfo {
  pub id: String,
  pub name: String,
  pub base_url: String,
  pub default: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChannelsResponse {
  pub channels: Vec<ChannelInfo>,
}

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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MediaKind {
  Movie,
  TV,
  Variety,
  Anime,
  Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaMetadata {
  pub channel: String,
  pub id: String,
  pub name: String,
  pub poster_url: String,
  pub release_year: u32,
  pub description: String,
  pub kind: MediaKind,
}

impl MediaMetadata {
  pub fn is_movie(&self) -> bool {
    self.kind == MediaKind::Movie
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMediaPlaylistRequest {
  pub channel: String,
  pub media_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaPlaylistItem {
  pub number: u32,
  pub text: String,
  pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaPlaylist {
  pub channel: String,
  pub media_id: String,
  pub items: Vec<MediaPlaylistItem>,
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
