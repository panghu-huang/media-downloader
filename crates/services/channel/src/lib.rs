mod common;
mod services;

use configuration::Configuration;
use protocol::channel::ChannelExt;
use protocol::channel::DownloadMediaRequest;
use protocol::channel::{GetMediaMetadataRequest, MediaMetadata};
use protocol::channel::{GetMediaPlaylistRequest, MediaPlaylist};
use protocol::channel::{SearchMediaRequest, SearchMediaResponse};
use protocol::tonic;
use protocol::tonic::{async_trait, Request, Response, Status};
use protocol::DownloadProgressReceiver;
use services::{DownloadMediaOptions, MediaChannelExt};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct ChannelService {
  destination_dir: PathBuf,
  channels: HashMap<String, Box<dyn MediaChannelExt>>,
  default_channel: String,
}

#[async_trait]
impl ChannelExt for ChannelService {
  type DownloadMediaStream = DownloadProgressReceiver;

  async fn download_media(
    &self,
    request: Request<DownloadMediaRequest>,
  ) -> tonic::Result<Response<Self::DownloadMediaStream>> {
    let request = request.into_inner();
    log::info!(
      "Downloading media {} ({:?})",
      request.media_id,
      request.number
    );

    let file_name = format!("{}-{}.mp4", request.media_id, request.number.unwrap_or(1));

    let options = DownloadMediaOptions {
      media_id: request.media_id,
      number: request.number,
      destination_path: self.destination_dir.join(file_name),
    };

    let channel = self.get_channel_by_name(&request.channel)?;

    let download_progress_receiver = channel
      .download_media(options)
      .await
      .map_err(|e| Status::internal(format!("Error occurred during download media: {}", e)))?;

    Ok(Response::new(download_progress_receiver))
  }

  async fn get_media_metadata(
    &self,
    request: Request<GetMediaMetadataRequest>,
  ) -> tonic::Result<Response<MediaMetadata>> {
    let request = request.into_inner();
    log::info!("Getting media metadata of {}", request.media_id);

    let channel = self.get_channel_by_name(&request.channel)?;

    let metadata = channel
      .get_media_metadata(&request.media_id)
      .await
      .map_err(|e| Status::internal(format!("Failed to get media metadata: {}", e)))?;

    Ok(Response::new(metadata))
  }

  async fn search_media(
    &self,
    request: Request<SearchMediaRequest>,
  ) -> tonic::Result<Response<SearchMediaResponse>> {
    let request = request.into_inner();
    log::info!("Searching media metadata of {}", request.keyword);

    let channel_name = request.channel.as_ref().unwrap_or(&self.default_channel);
    let channel = self.get_channel_by_name(channel_name)?;

    let search_result = channel
      .search_media(&request)
      .await
      .map_err(|e| Status::internal(format!("Failed to search media: {}", e)))?;

    Ok(Response::new(search_result))
  }

  async fn get_media_playlist(
    &self,
    request: Request<GetMediaPlaylistRequest>,
  ) -> tonic::Result<Response<MediaPlaylist>> {
    let request = request.into_inner();
    log::info!("Getting media playlist of {}", request.media_id);

    let channel = self.get_channel_by_name(&request.channel)?;

    let playlist = channel
      .get_media_playlist(&request.media_id)
      .await
      .map_err(|e| Status::internal(format!("Failed to get media playlist: {}", e)))?;

    Ok(Response::new(playlist))
  }
}

impl ChannelService {
  fn get_channel_by_name(&self, name: &str) -> tonic::Result<&dyn MediaChannelExt> {
    let channel = self
      .channels
      .get(name)
      .ok_or_else(|| Status::invalid_argument(format!("No channel '{}' found.", name)))?;

    log::info!("Found channel named '{}'", channel.channel_name());

    Ok(channel.borrow())
  }
}

impl ChannelService {
  pub fn new(config: &Configuration) -> Self {
    use self::services::unified::UnifiedMediaService;

    let mut channels = HashMap::new();

    for (channel_name, config) in &config.channel.unified_channels {
      let unified_channel = UnifiedMediaService::new(channel_name, config);

      log::info!(
        "Adding new unified channel {} with base URL {} ... ",
        channel_name,
        config.base_url,
      );

      channels.insert(
        channel_name.to_string(),
        Box::new(unified_channel) as Box<_>,
      );
    }

    Self {
      channels,
      default_channel: config.channel.default.clone(),
      destination_dir: destination_dir(),
    }
  }
}

const DOWNLOAD_DESTINATION_DIR: &str = "/downloads";

fn destination_dir() -> PathBuf {
  let preset_destination_dir = PathBuf::from(DOWNLOAD_DESTINATION_DIR);

  if preset_destination_dir.exists() {
    return preset_destination_dir;
  }

  let base_dir = std::env::current_dir()
    .or_else(|_| Ok(PathBuf::from(".")) as anyhow::Result<PathBuf>)
    .expect("Failed to detect download directory");

  base_dir.join("downloads")
}
