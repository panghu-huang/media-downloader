pub type DownloadMediaRequest = crate::channel::DownloadMediaRequest;

pub type GetMediaMetadataRequest = crate::channel::GetMediaMetadataRequest;

pub type MediaMetadata = crate::channel::MediaMetadata;

mod media_inner {
  include!("./pb/media.Media.rs");
}

pub use media_inner::{
  media_client::MediaClient,
  media_server::{Media as MediaExt, MediaServer},
};
