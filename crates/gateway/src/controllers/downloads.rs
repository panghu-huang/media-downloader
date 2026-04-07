use crate::state::AppState;
use axum::extract::State;
use axum::response::sse::{Event, KeepAlive, Sse};
use axum::response::Json;
use futures::stream::{self, Stream};
use futures::StreamExt;
use std::convert::Infallible;
use task_manager::DownloadTask;
use tokio_stream::wrappers::BroadcastStream;

/// Handler for `GET /api/v1/downloads`
pub async fn list_downloads(State(state): State<AppState>) -> Json<Vec<DownloadTask>> {
  Json(state.task_manager.list_tasks())
}

/// Handler for `GET /api/v1/downloads/events`
pub async fn download_events_sse(
  State(state): State<AppState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
  let tasks = state.task_manager.list_tasks();
  let rx = state.task_manager.subscribe();

  let init_data = serde_json::to_string(&tasks).unwrap_or_else(|_| "[]".to_string());
  let init_event = stream::once(async move {
    Ok(Event::default().event("init").data(init_data))
  });

  let update_events = BroadcastStream::new(rx).filter_map(|result| async move {
    match result {
      Ok(event) => {
        let data = serde_json::to_string(&event).ok()?;
        Some(Ok(Event::default().event("task_update").data(data)))
      }
      Err(_) => None,
    }
  });

  Sse::new(init_event.chain(update_events)).keep_alive(KeepAlive::default())
}
