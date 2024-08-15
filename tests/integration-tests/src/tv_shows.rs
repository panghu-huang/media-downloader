use crate::common::setup_testing;
use crate::response_to_json;
use axum::http::StatusCode;
use protocol::channel::TVShowMetadata;
use testing::request::Request;

#[tokio::test]
async fn test_get_tv_show_metadata() -> anyhow::Result<()> {
  let tv_show_id = 548;
  let gateway = setup_testing().await;

  let app = gateway.router();

  let url = format!("/api/v1/channels/xiaobao/tv_shows/{}", tv_show_id);

  let response = Request::get(&url).send(&app).await?;

  assert_eq!(response.status(), StatusCode::OK);
  let response: TVShowMetadata = response_to_json!(response);

  assert_eq!(response.id, tv_show_id.to_string());
  assert_eq!(response.name, "海贼王");
  assert_eq!(response.year, 1999);

  Ok(())
}

#[tokio::test]
async fn test_get_tv_show_metadata_by_unified_channel() -> anyhow::Result<()> {
  let tv_show_id = 9104;
  let gateway = setup_testing().await;

  let app = gateway.router();

  let url = format!("/api/v1/channels/huaweiba/tv_shows/{}", tv_show_id);

  let response = Request::get(&url).send(&app).await?;

  assert_eq!(response.status(), StatusCode::OK);
  let response: TVShowMetadata = response_to_json!(response);

  assert_eq!(response.id, tv_show_id.to_string());
  assert_eq!(response.name, "琅琊榜");
  assert_eq!(response.year, 2015);

  Ok(())
}

#[tokio::test]
async fn test_download_tv_show_by_unified_channel() -> anyhow::Result<()> {
  let tv_show_id = 9104;
  let gateway = setup_testing().await;

  let app = gateway.router();

  let response = Request::post("/api/v1/tv_shows/download")
    .body(serde_json::json!({
      "channel": "huaweiba",
      "tv_show_id": tv_show_id.to_string(),
      "tv_show_episode_number": 1,
    }))
    .send(&app)
    .await?;

  assert_eq!(response.status(), StatusCode::CREATED);

  Ok(())
}
