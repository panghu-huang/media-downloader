use crate::common::setup_testing;
use crate::response_to_json;
use axum::http::StatusCode;
use task_manager::{DownloadTask, TaskStatus};
use testing::request::Request;

#[tokio::test]
async fn test_list_downloads_empty() -> anyhow::Result<()> {
  let gateway = setup_testing().await;
  let app = gateway.router();

  let response = Request::get("/api/v1/downloads").send(&app).await?;

  assert_eq!(response.status(), StatusCode::OK);

  let tasks: Vec<DownloadTask> = response_to_json!(response);
  assert!(tasks.is_empty());

  Ok(())
}

#[tokio::test]
async fn test_list_downloads_with_tasks() -> anyhow::Result<()> {
  let gateway = setup_testing().await;

  // Create a task directly via the task manager
  let task_id = gateway.task_manager.create_task(
    "huaweiba".to_string(),
    "9104".to_string(),
    "琅琊榜".to_string(),
    None,
  );

  let app = gateway.router();
  let response = Request::get("/api/v1/downloads").send(&app).await?;

  assert_eq!(response.status(), StatusCode::OK);

  let tasks: Vec<DownloadTask> = response_to_json!(response);
  assert_eq!(tasks.len(), 1);
  assert_eq!(tasks[0].id, task_id);
  assert_eq!(tasks[0].channel, "huaweiba");
  assert_eq!(tasks[0].media_id, "9104");
  assert_eq!(tasks[0].media_name, "琅琊榜");
  assert_eq!(tasks[0].status, TaskStatus::Pending);
  assert_eq!(tasks[0].progress, 0);

  Ok(())
}

#[tokio::test]
async fn test_download_task_progress_updates() -> anyhow::Result<()> {
  let gateway = setup_testing().await;

  let task_id = gateway.task_manager.create_task(
    "heimuer".to_string(),
    "24405".to_string(),
    "海贼王".to_string(),
    Some(1),
  );

  gateway.task_manager.task_started(&task_id, 100);

  for _ in 0..50 {
    gateway.task_manager.task_segment_downloaded(&task_id);
  }

  let app = gateway.router();
  let response = Request::get("/api/v1/downloads").send(&app).await?;
  assert_eq!(response.status(), StatusCode::OK);

  let tasks: Vec<DownloadTask> = response_to_json!(response);
  assert_eq!(tasks.len(), 1);
  assert_eq!(tasks[0].status, TaskStatus::Downloading);
  assert_eq!(tasks[0].downloaded_segments, 50);
  assert_eq!(tasks[0].total_segments, Some(100));
  assert_eq!(tasks[0].progress, 50);

  Ok(())
}

#[tokio::test]
async fn test_download_task_completed() -> anyhow::Result<()> {
  let gateway = setup_testing().await;

  let task_id = gateway.task_manager.create_task(
    "huaweiba".to_string(),
    "1234".to_string(),
    "测试剧".to_string(),
    Some(3),
  );

  gateway.task_manager.task_started(&task_id, 10);
  for _ in 0..10 {
    gateway.task_manager.task_segment_downloaded(&task_id);
  }
  gateway.task_manager.task_transforming(&task_id);
  gateway.task_manager.task_completed(&task_id);

  let app = gateway.router();
  let response = Request::get("/api/v1/downloads").send(&app).await?;
  assert_eq!(response.status(), StatusCode::OK);

  let tasks: Vec<DownloadTask> = response_to_json!(response);
  assert_eq!(tasks.len(), 1);
  assert_eq!(tasks[0].status, TaskStatus::Completed);
  assert_eq!(tasks[0].progress, 100);

  Ok(())
}

#[tokio::test]
async fn test_download_task_failed() -> anyhow::Result<()> {
  let gateway = setup_testing().await;

  let task_id = gateway.task_manager.create_task(
    "huaweiba".to_string(),
    "5678".to_string(),
    "失败的剧".to_string(),
    None,
  );

  gateway.task_manager.task_failed(&task_id, "network error");

  let app = gateway.router();
  let response = Request::get("/api/v1/downloads").send(&app).await?;
  assert_eq!(response.status(), StatusCode::OK);

  let tasks: Vec<DownloadTask> = response_to_json!(response);
  assert_eq!(tasks.len(), 1);
  assert_eq!(tasks[0].status, TaskStatus::Failed);
  assert_eq!(tasks[0].error_message.as_deref(), Some("network error"));

  Ok(())
}

#[tokio::test]
async fn test_download_events_sse_returns_ok() -> anyhow::Result<()> {
  let gateway = setup_testing().await;
  let app = gateway.router();

  let response = Request::get("/api/v1/downloads/events").send(&app).await?;

  assert_eq!(response.status(), StatusCode::OK);

  use axum::http::StatusCode;
  let headers = response.headers().clone();
  assert_eq!(
    headers.get("content-type").and_then(|v| v.to_str().ok()),
    Some("text/event-stream")
  );

  Ok(())
}
