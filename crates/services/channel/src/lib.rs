mod common;
mod services;

use configuration::Configuration;
use protocol::channel::ChannelExt;
use protocol::channel::{DownloadTVShowRequest, DownloadTVShowResponse};
use protocol::tonic::{async_trait, Request, Response, Status};
use services::{DownloadTVShowOptions, MediaChannelExt};
use std::collections::HashMap;
use std::path::PathBuf;

pub struct ChannelService {
  destination_dir: PathBuf,
  channels: HashMap<String, Box<dyn MediaChannelExt>>,
}

#[async_trait]
impl ChannelExt for ChannelService {
  async fn download_tv_show(
    &self,
    request: Request<DownloadTVShowRequest>,
  ) -> Result<Response<DownloadTVShowResponse>, Status> {
    let request = request.into_inner();
    let destination_dir = self.destination_dir.clone();

    let options = DownloadTVShowOptions {
      tv_show_id: request.tv_show_id,
      tv_show_season_number: request.tv_show_season_number,
      tv_show_episode_number: request.tv_show_episode_number,
      destination_dir,
    };

    let channel = self.channels.get(&request.channel).ok_or_else(|| {
      Status::invalid_argument(format!("No channel '{}' found.", request.channel))
    })?;

    let res = channel
      .download_tv_show(options)
      .await
      .map_err(|e| Status::internal(format!("Error occurred during download TV show: {}", e)))?;

    Ok(Response::new(res))
  }
}

impl ChannelService {
  pub fn new(config: &Configuration) -> Self {
    let xiaobao = services::xiaobao::XiaobaoTV::new(&config.channels.xiaobao.host);

    let channels = [(
      "xiaobao".to_owned(),
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
