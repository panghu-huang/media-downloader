use protocol::media::BatchDownloadMediaRequest;
use protocol::media::DownloadMediaRequest;
use protocol::media::MediaExt;
use protocol::media::{GetMediaMetadataRequest, MediaMetadata};
use protocol::media::{GetMediaPlaylistRequest, MediaPlaylist};
use protocol::media::{SearchMediaRequest, SearchMediaResponse};
use protocol::tonic::{self, async_trait, Request, Response};
use rpc_client::RpcClient;
use std::ops::AddAssign;
use std::path::Path;
use std::path::PathBuf;

pub struct MediaService {
  media_dir: PathBuf,
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

    let mut channel_client = self.rpc_client.channel.clone();
    let metadata = channel_client
      .get_media_metadata(GetMediaMetadataRequest {
        channel: request.channel.clone(),
        media_id: request.media_id.clone(),
      })
      .await?
      .into_inner();

    tokio::spawn({
      let batch_request = request.clone();
      let media_dir = self.media_dir.clone();

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
          .and_then(|local_path| {
            Self::rename_media_file(metadata.clone(), &media_dir, start_number, &local_path)
          }) {
            log::info!(
              "Failed to download media {}(#{:?}): {}",
              request.media_id,
              start_number,
              err,
            );

            break;
          }
        }

        log::info!("Batch download completed");
      }
    });

    Ok(Response::new(protocol::Empty {}))
  }
}

impl MediaService {
  async fn download_media(
    mut channel_client: protocol::channel::ChannelClient<tonic::transport::Channel>,
    request: DownloadMediaRequest,
  ) -> anyhow::Result<PathBuf> {
    let res = channel_client.download_media(request).await?;

    log::info!("Successful to request download media");

    let mut total = None;
    let mut finished = 0;
    let mut progress = res.into_inner();

    while let Some(evt) = progress.message().await.unwrap() {
      match evt {
        protocol::DownloadProgressItem::Done { local_path, .. } => {
          log::info!("Done");
          return Ok(PathBuf::from(local_path));
        }
        protocol::DownloadProgressItem::Started {
          total_segments_of_media,
          ..
        } => {
          total = Some(total_segments_of_media);
        }
        protocol::DownloadProgressItem::SegmentDownloaded { .. } => {
          finished.add_assign(1);
          log::info!("Downloading ... {}/{}", finished, total.unwrap_or(0));
        }
        _ => {
          log::info!("Event {:#?}", evt);
        }
      };
    }

    anyhow::bail!("No done event received")
  }

  fn rename_media_file(
    metadata: MediaMetadata,
    media_dir: &Path,
    episode_number: u32,
    local_path: &Path,
  ) -> anyhow::Result<()> {
    let ext = local_path
      .extension()
      .ok_or(anyhow::anyhow!("Failed to parse extension from local path"))?
      .to_string_lossy()
      .to_string();

    let base_dir_name = format!("{} ({})", metadata.name, metadata.release_year);
    let season_dir_name = format!("Season {}", "01");
    let file_name = format!("{} S{}E{}.{}", metadata.name, "01", episode_number, ext);

    let new_local_path = media_dir
      .join(base_dir_name)
      .join(season_dir_name)
      .join(file_name);

    log::info!(
      "Rename file from {} to {}",
      local_path.to_string_lossy().to_string(),
      new_local_path.to_string_lossy().to_string()
    );

    std::fs::create_dir_all(new_local_path.parent().unwrap())?;

    std::fs::rename(local_path, new_local_path)?;

    Ok(())
  }
}

impl MediaService {
  pub fn new(rpc_client: &RpcClient) -> Self {
    Self {
      media_dir: media_dir(),
      rpc_client: rpc_client.clone(),
    }
  }
}

fn media_dir() -> PathBuf {
  let base_dir = std::env::current_dir()
    .or_else(|_| Ok(PathBuf::from(".")) as anyhow::Result<PathBuf>)
    .expect("Failed to detect media directory");

  base_dir.join("media")
}
