use protocol::media::DownloadTVShowRequest;
use protocol::media::MediaExt;
use protocol::tonic::{async_trait, Request, Response, Status};
use rpc_client::RpcClient;

pub struct MediaService {
  rpc_client: RpcClient,
}

#[async_trait]
impl MediaExt for MediaService {
  async fn download_tv_show(
    &self,
    request: Request<DownloadTVShowRequest>,
  ) -> Result<Response<protocol::Empty>, Status> {
    let request = request.into_inner();

    let mut channel_client = self.rpc_client.channel.clone();

    tokio::spawn(async move {
      if let Err(err) = channel_client.download_tv_show(request.clone()).await {
        log::info!(
          "Failed to download TV show {}(E{}S{}): {}",
          request.tv_show_id,
          request.tv_show_sesson_number,
          request.tv_show_episode_number,
          err,
        );
      }
    });

    Ok(Response::new(protocol::Empty {}))
  }
}

impl MediaService {
  pub fn new(rpc_client: &RpcClient) -> Self {
    Self {
      rpc_client: rpc_client.clone(),
    }
  }
}
