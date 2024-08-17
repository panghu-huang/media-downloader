use protocol::media::DownloadMediaRequest;
use protocol::media::MediaExt;
use protocol::media::{GetMediaMetadataRequest, MediaMetadata};
use protocol::tonic::{self, async_trait, Request, Response};
use rpc_client::RpcClient;

pub struct MediaService {
  rpc_client: RpcClient,
}

#[async_trait]
impl MediaExt for MediaService {
  async fn download_media(
    &self,
    request: Request<DownloadMediaRequest>,
  ) -> tonic::Result<Response<protocol::Empty>> {
    let request = request.into_inner();
    log::info!(
      "Downloading media {} (#{:?})",
      request.media_id,
      request.number
    );

    let mut channel_client = self.rpc_client.channel.clone();

    tokio::spawn(async move {
      let res = channel_client.download_media(request.clone()).await;

      match res {
        Ok(res) => {
          log::info!("successful to request download tv show");

          let mut progress = res.into_inner();

          while let Some(evt) = progress.message().await.unwrap() {
            log::info!("Event {:#?}", evt);
          }

          log::info!("Done");
        }
        Err(err) => {
          log::info!(
            "Failed to download media {}(#{:?}): {}",
            request.media_id,
            request.number,
            err,
          );
        }
      }
    });

    Ok(Response::new(protocol::Empty {}))
  }

  async fn get_media_metadata(
    &self,
    request: Request<GetMediaMetadataRequest>,
  ) -> tonic::Result<Response<MediaMetadata>> {
    let request = request.into_inner();

    let mut channel_client = self.rpc_client.channel.clone();

    let res = channel_client
      .get_media_metadata(request.clone())
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
