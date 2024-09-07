use protocol::media::BatchDownloadMediaRequest;
use protocol::media::DownloadMediaRequest;
use protocol::media::MediaExt;
use protocol::media::{GetMediaMetadataRequest, MediaMetadata};
use protocol::media::{GetMediaPlaylistRequest, MediaPlaylist};
use protocol::media::{SearchMediaRequest, SearchMediaResponse};
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

    tokio::spawn({
      let channel_client = self.rpc_client.channel.clone();
      async move {
        if let Err(err) = Self::download_media(channel_client, request.clone()).await {
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

  async fn search_media(
    &self,
    request: Request<SearchMediaRequest>,
  ) -> tonic::Result<Response<SearchMediaResponse>> {
    let request = request.into_inner();

    let mut channel_client = self.rpc_client.channel.clone();

    let res = channel_client
      .search_media(request.clone())
      .await?
      .into_inner();

    Ok(Response::new(res))
  }

  async fn get_media_playlist(
    &self,
    request: Request<GetMediaPlaylistRequest>,
  ) -> tonic::Result<Response<MediaPlaylist>> {
    let request = request.into_inner();

    let mut channel_client = self.rpc_client.channel.clone();

    let res = channel_client
      .get_media_playlist(request.clone())
      .await?
      .into_inner();

    Ok(Response::new(res))
  }

  async fn batch_download_media(
    &self,
    request: Request<BatchDownloadMediaRequest>,
  ) -> tonic::Result<Response<protocol::Empty>> {
    // TODO: Validate download range
    let request = request.into_inner();
    log::info!(
      "Downloading media {} in batch (#{:?} - {})",
      request.media_id,
      request.start_number,
      request.count,
    );

    tokio::spawn({
      let channel_client = self.rpc_client.channel.clone();
      let batch_request = request.clone();

      async move {
        for idx in 0..batch_request.count {
          let start_number = batch_request.start_number + (idx as u32);

          if let Err(err) = Self::download_media(
            channel_client.clone(),
            DownloadMediaRequest {
              channel: batch_request.channel.clone(),
              media_id: batch_request.media_id.clone(),
              number: Some(start_number),
            },
          )
          .await
          {
            log::info!(
              "Failed to download media {}(#{:?}): {}",
              request.media_id,
              start_number,
              err,
            );
          }
        }
      }
    });

    Ok(Response::new(protocol::Empty {}))
  }
}

impl MediaService {
  async fn download_media(
    mut channel_client: protocol::channel::ChannelClient<tonic::transport::Channel>,
    request: DownloadMediaRequest,
  ) -> anyhow::Result<()> {
    let res = channel_client.download_media(request).await?;

    log::info!("successful to request download media");

    let mut progress = res.into_inner();

    while let Some(evt) = progress.message().await.unwrap() {
      log::info!("Event {:#?}", evt);
    }

    log::info!("Done");

    Ok(())
  }
}

impl MediaService {
  pub fn new(rpc_client: &RpcClient) -> Self {
    Self {
      rpc_client: rpc_client.clone(),
    }
  }
}
