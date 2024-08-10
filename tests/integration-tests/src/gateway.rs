use crate::common::setup_testing;
use crate::response_to_json;
use axum::http::StatusCode;
use serde_json::json;
use testing::request::Request;

#[tokio::test]
async fn test_gateway_404() -> anyhow::Result<()> {
  let gateway = setup_testing().await;

  let app = gateway.router();

  let response = Request::get("/api/v1/not-found").send(&app).await?;

  assert_eq!(response.status(), StatusCode::NOT_FOUND);

  let response: serde_json::Value = response_to_json!(response);

  assert_eq!(
    response,
    json!({
      "status": 404,
      "message": "The requested resource could not be found. Please check your request and try again.",
    })
  );

  Ok(())
}
