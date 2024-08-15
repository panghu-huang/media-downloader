mod common;
mod services;

use configuration::Configuration;
use protocol::channel::ChannelExt;
use protocol::channel::DownloadTVShowRequest;
use protocol::channel::{GetTVShowMetadataRequest, TVShowMetadata};
use protocol::tonic;
use protocol::tonic::{async_trait, Request, Response, Status};
use protocol::DownloadProgressReceiver;
use services::{DownloadTVShowOptions, MediaChannelExt};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct ChannelService {
  destination_dir: PathBuf,
  channels: HashMap<String, Box<dyn MediaChannelExt>>,
}

#[async_trait]
impl ChannelExt for ChannelService {
  type DownloadTvShowStream = DownloadProgressReceiver;

  async fn download_tv_show(
    &self,
    request: Request<DownloadTVShowRequest>,
  ) -> tonic::Result<Response<Self::DownloadTvShowStream>> {
    let request = request.into_inner();
    log::info!(
      "Downloading TV show {} (E{})",
      request.tv_show_id,
      request.tv_show_episode_number
    );

    let file_name = format!(
      "{}-{}.mp4",
      request.tv_show_id, request.tv_show_episode_number
    );

    let options = DownloadTVShowOptions {
      tv_show_id: request.tv_show_id,
      tv_show_episode_number: request.tv_show_episode_number,
      destination_path: self.destination_dir.join(file_name),
    };

    let channel = self.get_channel_by_name(&request.channel)?;

    let download_progress_receiver = channel
      .download_tv_show(options)
      .await
      .map_err(|e| Status::internal(format!("Error occurred during download TV show: {}", e)))?;

    Ok(Response::new(download_progress_receiver))
  }

  async fn get_tv_show_metadata(
    &self,
    request: Request<GetTVShowMetadataRequest>,
  ) -> tonic::Result<Response<TVShowMetadata>> {
    let request = request.into_inner();
    log::info!("Getting TV show metadata of {}", request.tv_show_id);

    let channel = self.get_channel_by_name(&request.channel)?;

    let metadata = channel
      .get_tv_show_metadata(&request.tv_show_id)
      .await
      .map_err(|e| Status::internal(format!("Failed to get TV show metadata: {}", e)))?;

    Ok(Response::new(metadata))
  }
}

impl ChannelService {
  fn get_channel_by_name(&self, name: &str) -> tonic::Result<&dyn MediaChannelExt> {
    let channel = self
      .channels
      .get(name)
      .ok_or_else(|| Status::invalid_argument(format!("No channel '{}' found.", name)))?;

    Ok(channel.borrow())
  }
}

impl ChannelService {
  pub fn new(config: &Configuration) -> Self {
    use self::services::unified::UnifiedMediaService;
    use self::services::xiaobao::XiaobaoTV;

    let xiaobao = XiaobaoTV::new(&config.channels.xiaobao.host);

    let mut channels: HashMap<String, Box<dyn MediaChannelExt>> = [(
      xiaobao.channel_name().to_owned(),
      Box::new(xiaobao) as Box<dyn MediaChannelExt>,
    )]
    .into_iter()
    .collect();

    for (channel_name, config) in &config.unified_channels {
      let unified_channel = UnifiedMediaService::new(channel_name, &config.base_url);

      log::info!(
        "Adding new unified channel {} with base URL {} ... ",
        channel_name,
        config.base_url,
      );

      channels.insert(channel_name.to_string(), Box::new(unified_channel));
    }

    Self {
      channels,
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
