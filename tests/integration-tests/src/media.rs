use crate::common::setup_testing;
use crate::response_to_json;
use axum::http::StatusCode;
use protocol::channel::{MediaKind, MediaMetadata};
use testing::request::Request;

#[tokio::test]
async fn test_get_media_metadata_by_huaweiba() -> anyhow::Result<()> {
  let channel_name = "huaweiba";
  let media_id = 9104;
  let gateway = setup_testing().await;

  let app = gateway.router();

  let url = format!("/api/v1/channels/{}/media/{}", channel_name, media_id);

  let response = Request::get(&url).send(&app).await?;

  assert_eq!(response.status(), StatusCode::OK);

  let response: MediaMetadata = response_to_json!(response);

  assert_eq!(response.id, media_id.to_string());
  assert_eq!(response.name, "琅琊榜");
  assert_eq!(response.release_year, 2015);
  assert_eq!(response.kind, MediaKind::TV);

  Ok(())
}

#[tokio::test]
async fn test_get_media_metadata_by_heimuer() -> anyhow::Result<()> {
  let channel_name = "heimuer";
  let media_id = 24405;
  let gateway = setup_testing().await;

  let app = gateway.router();

  let url = format!("/api/v1/channels/{}/media/{}", channel_name, media_id);

  let response = Request::get(&url).send(&app).await?;

  assert_eq!(response.status(), StatusCode::OK);

  let response: MediaMetadata = response_to_json!(response);

  assert_eq!(response.id, media_id.to_string());
  assert_eq!(response.name, "海贼王");
  assert_eq!(response.release_year, 1999);
  assert_eq!(response.kind, MediaKind::Anime);

  Ok(())
}
