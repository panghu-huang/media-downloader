use crate::extracts::{JsonBody, RpcClient};
use axum::extract::{Json, Path, Query};
use axum::http::StatusCode;
use protocol::media::BatchDownloadMediaRequest;
use protocol::media::DownloadMediaRequest;
use protocol::media::{GetMediaMetadataRequest, MediaMetadata};
use protocol::media::{SearchMediaRequest, SearchMediaResponse};

#[derive(serde::Deserialize)]
pub struct SearchMediaQuery {
  pub channel: Option<String>,
  pub keyword: String,
  pub page: Option<u32>,
  pub page_size: Option<u32>,
}

/// Handler for `POST /api/v1/media/download`
pub async fn download_media(
  RpcClient(rpc_client): RpcClient,
  JsonBody(request): JsonBody<DownloadMediaRequest>,
) -> crate::Result<StatusCode> {
  let mut media_client = rpc_client.media.clone();

  media_client.download_media(request).await?;

  Ok(StatusCode::CREATED)
}

/// Handler for `POST /api/v1/media/batch_download`
pub async fn batch_download_media(
  RpcClient(rpc_client): RpcClient,
  JsonBody(request): JsonBody<BatchDownloadMediaRequest>,
) -> crate::Result<StatusCode> {
  let mut media_client = rpc_client.media.clone();

  media_client.batch_download_media(request).await?;

  Ok(StatusCode::CREATED)
}

/// Handler for `GET /api/v1/channels/:channel_name/media/:media_id`
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

/// Handler for `GET /api/v1/media/search`
pub async fn search_media(
  RpcClient(rpc_client): RpcClient,
  Query(query): Query<SearchMediaQuery>,
) -> crate::Result<Json<SearchMediaResponse>> {
  let mut media_client = rpc_client.media.clone();

  let request = SearchMediaRequest {
    keyword: query.keyword,
    channel: query.channel,
    page: query.page.unwrap_or(1),
    page_size: query.page_size.unwrap_or(20),
  };

  let res = media_client.search_media(request).await?.into_inner();

  Ok(Json(res))
}
