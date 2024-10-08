use serde::{Deserialize, Serialize};

pub type DownloadMediaRequest = crate::channel::DownloadMediaRequest;

pub type GetMediaMetadataRequest = crate::channel::GetMediaMetadataRequest;

pub type GetMediaPlaylistRequest = crate::channel::GetMediaPlaylistRequest;

pub type MediaPlaylist = crate::channel::MediaPlaylist;

pub type SearchMediaRequest = crate::channel::SearchMediaRequest;

pub type SearchMediaResponse = crate::channel::SearchMediaResponse;

pub type MediaMetadata = crate::channel::MediaMetadata;

#[derive(Deserialize, Serialize, Clone)]
pub struct BatchDownloadMediaRequest {
  pub channel: String,
  pub media_id: String,
  pub start_number: u32,
  pub count: u8,
}

mod media_inner {
  include!("./pb/media.Media.rs");
}

pub use media_inner::{
  media_client::MediaClient,
  media_server::{Media as MediaExt, MediaServer},
};
