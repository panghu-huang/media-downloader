use protocol::media::DownloadTVShowRequest;
use protocol::media::MediaExt;
use protocol::media::{GetTVShowMetadataRequest, TVShowMetadata};
use protocol::tonic::{self, async_trait, Request, Response};
use rpc_client::RpcClient;

pub struct MediaService {
  rpc_client: RpcClient,
}

#[async_trait]
impl MediaExt for MediaService {
  async fn download_tv_show(
    &self,
    request: Request<DownloadTVShowRequest>,
  ) -> tonic::Result<Response<protocol::Empty>> {
    let request = request.into_inner();
    log::info!(
      "Downloading TV show {} (S{}E{})",
      request.tv_show_id,
      request.tv_show_season_number,
      request.tv_show_episode_number
    );

    let mut channel_client = self.rpc_client.channel.clone();

    tokio::spawn(async move {
      let res = channel_client.download_tv_show(request.clone()).await;

      match res {
        Ok(res) => {
          log::info!("successful to request download tv show");

          let mut progress = res.into_inner();

          while let Some(evt) = progress.message().await.unwrap() {
            log::info!("Event {:#?}", evt);
          }
        }
        Err(err) => {
          log::info!(
            "Failed to download TV show {}(S{}E{}): {}",
            request.tv_show_id,
            request.tv_show_season_number,
            request.tv_show_episode_number,
            err,
          );
        }
      }
    });

    Ok(Response::new(protocol::Empty {}))
  }

  async fn get_tv_show_metadata(
    &self,
    request: Request<GetTVShowMetadataRequest>,
  ) -> tonic::Result<Response<TVShowMetadata>> {
    let request = request.into_inner();

    let mut channel_client = self.rpc_client.channel.clone();

    let res = channel_client
      .get_tv_show_metadata(request.clone())
      .await?
      .into_inner();

    Ok(Response::new(res))
  }
}

impl MediaService {
  pub fn new(rpc_client: &RpcClient) -> Self {
    Self {
      rpc_client: rpc_client.clone(),
    }
  }
}
