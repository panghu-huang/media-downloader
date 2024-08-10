use crate::{AppState, Result};
use axum::extract::{Json, State};
use axum::http::StatusCode;
use protocol::channel::DownloadTVShowResponse;

/// Handler for `POST /api/v1/users`
pub async fn create_download_tv_show_request(
  State(_state): State<AppState>,
) -> Result<(StatusCode, Json<DownloadTVShowResponse>)> {
  todo!()
}
