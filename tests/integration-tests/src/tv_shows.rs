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
async fn test_get_tv_show_metadata_by_huaweiba() -> anyhow::Result<()> {
  let channel_name = "huaweiba";
  let tv_show_id = 9104;
  let gateway = setup_testing().await;

  let app = gateway.router();

  let url = format!("/api/v1/channels/{}/tv_shows/{}", channel_name, tv_show_id);

  let response = Request::get(&url).send(&app).await?;

  assert_eq!(response.status(), StatusCode::OK);
  let response: TVShowMetadata = response_to_json!(response);

  assert_eq!(response.id, tv_show_id.to_string());
  assert_eq!(response.name, "琅琊榜");
  assert_eq!(response.year, 2015);

  Ok(())
}

#[tokio::test]
async fn test_get_tv_show_metadata_by_heimuer() -> anyhow::Result<()> {
  let channel_name = "heimuer";
  let tv_show_id = 24405;
  let gateway = setup_testing().await;

  let app = gateway.router();

  let url = format!("/api/v1/channels/{}/tv_shows/{}", channel_name, tv_show_id);

  let response = Request::get(&url).send(&app).await?;

  assert_eq!(response.status(), StatusCode::OK);
  let response: TVShowMetadata = response_to_json!(response);

  assert_eq!(response.id, tv_show_id.to_string());
  assert_eq!(response.name, "海贼王");
  assert_eq!(response.year, 1999);

  Ok(())
}
