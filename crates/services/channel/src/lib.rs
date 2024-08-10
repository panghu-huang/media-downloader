mod common;
mod services;

use protocol::channel::ChannelExt;
use protocol::channel::{DownloadTVShowRequest, DownloadTVShowResponse};
use protocol::tonic::{async_trait, Request, Response, Status};

#[derive(Default)]
pub struct ChannelService;

#[async_trait]
impl ChannelExt for ChannelService {
  async fn download_tv_show(
    &self,
    request: Request<DownloadTVShowRequest>,
  ) -> Result<Response<DownloadTVShowResponse>, Status> {
    todo!()
  }
}

impl ChannelService {
  pub fn new() -> Self {
    Default::default()
  }
}
