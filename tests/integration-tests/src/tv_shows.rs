use crate::common::setup_testing;
use crate::response_to_json;
use protocol::channel::TVShowMetadata;
use testing::request::Request;

#[tokio::test]
async fn test_get_tv_show_metadata() -> anyhow::Result<()> {
  let tv_show_id = 548;
  let gateway = setup_testing().await;

  let app = gateway.router();

  let url = format!("/api/v1/channels/xiaobao/tv_shows/{}", tv_show_id);

  let response = Request::get(&url).send(&app).await?;

  let response: TVShowMetadata = response_to_json!(response);

  assert_eq!(response.id, tv_show_id.to_string());
  assert_eq!(response.name, "海贼王");
  assert_eq!(response.year, 1999);

  Ok(())
}
