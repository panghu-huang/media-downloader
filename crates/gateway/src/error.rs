use axum::http::StatusCode;
use axum::response::{IntoResponse, Json, Response};
use protocol::tonic;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct AppResponse {
  pub status: u16,
  pub message: String,
}

#[derive(Debug)]
pub enum AppError {
  InternalServerError(anyhow::Error),
  ProxyError(anyhow::Error),
  DatabaseError(anyhow::Error),
  ValidationError(String),
  NotFound(String),
  BadRequest(String),
  Unauthorized(String),
  UnsupportedMediaType(String),
  Forbidden,
}

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
  fn into_response(self) -> Response {
    let (status, message) = match self {
      Self::InternalServerError(err) => {
        log::error!("Internal server error: {:?}", err);
        (
          StatusCode::INTERNAL_SERVER_ERROR,
          "Internal server error".to_string(),
        )
      }
      Self::DatabaseError(err) => {
        log::error!("Database error: {:?}", err);
        (
          StatusCode::INTERNAL_SERVER_ERROR,
          "Database error".to_string(),
        )
      }
      Self::ProxyError(err) => {
        log::error!("Bad gateway: {:?}", err);
        (StatusCode::BAD_GATEWAY, "Bad gateway".to_string())
      }
      Self::UnsupportedMediaType(message) => (StatusCode::UNSUPPORTED_MEDIA_TYPE, message),
      Self::NotFound(message) => (StatusCode::NOT_FOUND, message),
      Self::BadRequest(message) => (StatusCode::BAD_REQUEST, message),
      Self::Unauthorized(message) => (StatusCode::UNAUTHORIZED, message),
      Self::Forbidden => (StatusCode::FORBIDDEN, "Forbidden".to_string()),
      Self::ValidationError(message) => (StatusCode::UNPROCESSABLE_ENTITY, message),
    };

    let response = AppResponse {
      status: status.as_u16(),
      message: message.to_string(),
    };

    log::info!("Error response: {:?}", response);

    let json = Json(response);

    (status, json).into_response()
  }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
// impl<E> From<E> for AppError
// where
//   E: Into<anyhow::Error>,
// {
//   fn from(err: E) -> Self {
//     Self::InternalServerError(err.into())
//   }
// }
impl From<tonic::Status> for AppError {
  fn from(status: tonic::Status) -> Self {
    match status.code() {
      tonic::Code::NotFound => AppError::not_found(status.message()),
      tonic::Code::InvalidArgument => AppError::bad_request(status.message()),
      tonic::Code::PermissionDenied => AppError::forbidden(),
      tonic::Code::Unauthenticated => AppError::unauthorized(status.message()),
      tonic::Code::Internal => AppError::internal_server_error(status.message()),
      _ => AppError::internal_server_error(status.message()),
    }
  }
}

impl AppError {
  pub fn not_found(message: impl Into<String>) -> Self {
    Self::NotFound(message.into())
  }

  pub fn validation_error(message: impl Into<String>) -> Self {
    Self::ValidationError(message.into())
  }

  pub fn bad_request(message: impl Into<String>) -> Self {
    Self::BadRequest(message.into())
  }

  pub fn unauthorized(message: impl Into<String>) -> Self {
    Self::Unauthorized(message.into())
  }

  pub fn forbidden() -> Self {
    Self::Forbidden
  }

  pub fn internal_server_error(message: impl Into<String>) -> Self {
    Self::InternalServerError(anyhow::anyhow!(message.into()))
  }

  pub fn database_error(message: impl Into<String>) -> Self {
    Self::DatabaseError(anyhow::anyhow!(message.into()))
  }

  pub fn proxy_error(message: impl Into<String>) -> Self {
    Self::ProxyError(anyhow::anyhow!(message.into()))
  }

  pub fn unsupported_media_type(message: impl Into<String>) -> Self {
    Self::UnsupportedMediaType(message.into())
  }
}

impl From<AppError> for Box<dyn std::error::Error> {
  fn from(err: AppError) -> Box<dyn std::error::Error> {
    match err {
      AppError::InternalServerError(err) => err.into(),
      AppError::DatabaseError(err) => err.into(),
      AppError::ValidationError(message) => anyhow::anyhow!(message).into(),
      AppError::NotFound(message) => anyhow::anyhow!(message).into(),
      AppError::BadRequest(message) => anyhow::anyhow!(message).into(),
      AppError::Unauthorized(message) => anyhow::anyhow!(message).into(),
      AppError::ProxyError(message) => anyhow::anyhow!(message).into(),
      AppError::UnsupportedMediaType(message) => anyhow::anyhow!(message).into(),
      AppError::Forbidden => {
        anyhow::anyhow!("Forbidden: You don't have permission to access this resource").into()
      }
    }
  }
}

impl fmt::Display for AppError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::InternalServerError(err) => write!(f, "Internal server error: {:?}", err),
      Self::DatabaseError(err) => write!(f, "Database error: {:?}", err),
      Self::ValidationError(message) => write!(f, "{}", message),
      Self::NotFound(message) => write!(f, "{}", message),
      Self::BadRequest(message) => write!(f, "{}", message),
      Self::Unauthorized(message) => write!(f, "{}", message),
      Self::ProxyError(message) => write!(f, "{}", message),
      Self::UnsupportedMediaType(message) => write!(f, "{}", message),
      Self::Forbidden => write!(
        f,
        "Forbidden: You don't have permission to access this resource"
      ),
    }
  }
}
