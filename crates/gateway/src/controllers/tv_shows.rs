use crate::extracts::{JsonBody, RpcClient};
use axum::extract::{Json, Path};
use axum::http::StatusCode;
use protocol::media::DownloadTVShowRequest;
use protocol::media::{GetTVShowMetadataRequest, TVShowMetadata};

/// Handler for `POST /api/v1/tv_shows/download`
pub async fn download_tv_show(
  RpcClient(rpc_client): RpcClient,
  JsonBody(request): JsonBody<DownloadTVShowRequest>,
) -> crate::Result<StatusCode> {
  let mut media_client = rpc_client.media.clone();

  media_client.download_tv_show(request).await?;

  Ok(StatusCode::CREATED)
}

/// Handler for `GET /api/v1/channels/:channel_name/tv_shows/:tv_show_id`
pub async fn get_tv_show_metadata(
  RpcClient(rpc_client): RpcClient,
  Path((channel, tv_show_id)): Path<(String, String)>,
) -> crate::Result<Json<TVShowMetadata>> {
  let mut media_client = rpc_client.media.clone();

  let res = media_client
    .get_tv_show_metadata(GetTVShowMetadataRequest {
      channel,
      tv_show_id,
    })
    .await?
    .into_inner();

  Ok(Json(res))
}
