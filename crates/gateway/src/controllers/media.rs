use crate::extracts::{JsonBody, RpcClient};
use axum::extract::{Json, Path};
use axum::http::StatusCode;
use protocol::media::DownloadMediaRequest;
use protocol::media::{GetMediaMetadataRequest, MediaMetadata};

/// Handler for `POST /api/v1/media/download`
pub async fn download_media(
  RpcClient(rpc_client): RpcClient,
  JsonBody(request): JsonBody<DownloadMediaRequest>,
) -> crate::Result<StatusCode> {
  let mut media_client = rpc_client.media.clone();

  media_client.download_media(request).await?;

  Ok(StatusCode::CREATED)
}

/// Handler for `GET /api/v1/channels/:channel_name/tv_shows/:tv_show_id`
pub async fn get_media_metadata(
  RpcClient(rpc_client): RpcClient,
  Path((channel, media_id)): Path<(String, String)>,
) -> crate::Result<Json<MediaMetadata>> {
  let mut media_client = rpc_client.media.clone();

  let res = media_client
    .get_media_metadata(GetMediaMetadataRequest { channel, media_id })
    .await?
    .into_inner();

  Ok(Json(res))
}
