mod logging;
mod revision;

pub use logging::logging;
pub use revision::revision;

#[macro_export]
macro_rules! request_from_parts {
  ($body: ident, $parts: ident) => {{
    use axum::body::Body;
    use http_body_util::BodyExt;
    use $crate::error::AppError;

    // this wont work if the body is an long running stream
    let bytes = $body
      .collect()
      .await
      .map_err(|err| {
        AppError::internal_server_error(format!("Failed to read body of request: {}", err))
      })?
      .to_bytes();

    let req = Request::from_parts($parts, Body::from(bytes));

    req
  }};
}
