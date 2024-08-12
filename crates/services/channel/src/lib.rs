mod common;
mod services;

use configuration::Configuration;
use protocol::channel::ChannelExt;
use protocol::channel::{DownloadProgress, DownloadTVShowRequest};
use protocol::channel::{GetTVShowMetadataRequest, TVShowMetadata};
use protocol::tonic;
use protocol::tonic::{async_trait, Request, Response, Status};
use services::{DownloadTVShowOptions, MediaChannelExt};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::path::PathBuf;
use std::pin::Pin;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::Stream;
use tokio_stream::StreamExt;

pub struct ChannelService {
  destination_dir: PathBuf,
  channels: HashMap<String, Box<dyn MediaChannelExt>>,
}

#[async_trait]
impl ChannelExt for ChannelService {
  // TODO: improve this
  type DownloadTvShowStream = Pin<Box<dyn Stream<Item = tonic::Result<DownloadProgress>> + Send>>;

  async fn download_tv_show(
    &self,
    request: Request<DownloadTVShowRequest>,
  ) -> tonic::Result<Response<Self::DownloadTvShowStream>> {
    let request = request.into_inner();
    log::info!(
      "Downloading TV show {} (S{}E{})",
      request.tv_show_id,
      request.tv_show_season_number,
      request.tv_show_episode_number
    );

    let file_name = format!(
      "{}-{}-{}.mp4",
      request.tv_show_id, request.tv_show_season_number, request.tv_show_episode_number
    );

    let options = DownloadTVShowOptions {
      tv_show_id: request.tv_show_id,
      tv_show_season_number: request.tv_show_season_number,
      tv_show_episode_number: request.tv_show_episode_number,
      destination_path: self.destination_dir.join(file_name),
    };

    let channel = self.get_channel_by_name(&request.channel)?;

    let download_progress = channel
      .download_tv_show(options)
      .await
      .map_err(|e| Status::internal(format!("Error occurred during download TV show: {}", e)))?;

    Ok(Response::new(Box::pin(
      ReceiverStream::new(download_progress).map(Ok),
    )))
  }

  async fn get_tv_show_metadata(
    &self,
    request: Request<GetTVShowMetadataRequest>,
  ) -> tonic::Result<Response<TVShowMetadata>> {
    let request = request.into_inner();

    let channel = self.get_channel_by_name(&request.channel)?;

    let metadata = channel
      .get_tv_show_metadata(&request.tv_show_id, request.tv_show_season_number)
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
    let xiaobao = services::xiaobao::XiaobaoTV::new(&config.channels.xiaobao.host);

    let channels = [(
      xiaobao.channel_name().to_owned(),
      Box::new(xiaobao) as Box<dyn MediaChannelExt>,
    )]
    .into_iter()
    .collect();

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
