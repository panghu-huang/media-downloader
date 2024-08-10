use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use std::time::Instant;

pub async fn logging(req: Request, next: Next) -> Response {
  let method = req.method().clone();
  let path = req
    .uri()
    .path_and_query()
    .cloned()
    .map(|x| x.to_string())
    .unwrap_or(req.uri().path().to_string());

  log::info!("{} {}", method, path);
  let start = Instant::now();

  let res = next.run(req).await;

  let elapsed = start.elapsed().as_millis();

  log::info!(
    "{} {} returned {} in {}ms",
    method,
    path,
    res.status(),
    elapsed
  );

  res
}
