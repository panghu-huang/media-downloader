use crate::{AppState, Result};
use axum::extract::State;
use axum::http::StatusCode;

/// Handler for `POST /api/v1/tv_shows/download`
pub async fn download_tv_show(State(_state): State<AppState>) -> Result<StatusCode> {
  todo!()
}
