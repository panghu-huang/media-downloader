use axum::extract::Request;
use axum::http::HeaderValue;
use axum::middleware::Next;
use axum::response::Response;
use std::env;

pub async fn revision(req: Request, next: Next) -> Response {
  let mut res = next.run(req).await;

  if let Ok(revision) = env::var("REVISION") {
    if let Ok(revision) = HeaderValue::from_str(&revision) {
      res.headers_mut().insert("X-Revision", revision);
    }
  }

  res
}
