pub type DownloadTVShowRequest = crate::channel::DownloadTVShowRequest;

pub type GetTVShowMetadataRequest = crate::channel::GetTVShowMetadataRequest;

pub type TVShowMetadata = crate::channel::TVShowMetadata;

mod media_inner {
  include!("./pb/media.Media.rs");
}

pub use media_inner::{
  media_client::MediaClient,
  media_server::{Media as MediaExt, MediaServer},
};
