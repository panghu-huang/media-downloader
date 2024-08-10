use crate::error::AppError;
use axum::extract::rejection::JsonRejection;
use axum::extract::{FromRequest, Request};
use axum::http::header::CONTENT_TYPE;

pub struct JsonBody<T>(pub T);

#[axum::async_trait]
impl<S, T> FromRequest<S> for JsonBody<T>
where
  axum::Json<T>: FromRequest<S, Rejection = JsonRejection>,
  S: Send + Sync,
{
  type Rejection = AppError;

  async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
    let content_type_header = req.headers().get(CONTENT_TYPE);
    let content_type = content_type_header.and_then(|value| value.to_str().ok());

    if let Some(content_type) = content_type {
      if content_type.contains("application/json") {
        let (parts, body) = req.into_parts();

        let req = Request::from_parts(parts, body);

        let res = match axum::Json::<T>::from_request(req, state).await {
          Ok(value) => Ok(Self(value.0)),
          // convert the error from `axum::Json` into whatever we want
          Err(err) => Err(AppError::validation_error(format!(
            "Failed to parse JSON: {}",
            err
          ))),
        };

        return res;
      }
    }

    Err(AppError::unsupported_media_type(
      "Invalid `Content-Type` header",
    ))
  }
}
