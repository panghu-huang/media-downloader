use axum::body::Body;
use axum::http::{Method, Request as AxumRequest, Response};
use axum::Router;
use std::collections::HashMap;
use tower::ServiceExt;

#[derive(Debug, Clone, Default)]
pub struct Request {
  pub uri: String,
  pub method: Method,
  pub body: Option<serde_json::Value>,
  pub headers: HashMap<String, String>,
}

impl Request {
  pub fn get(uri: &str) -> Self {
    Self {
      uri: uri.to_string(),
      method: Method::GET,
      ..Default::default()
    }
  }

  pub fn post(uri: &str) -> Self {
    Self {
      uri: uri.to_string(),
      method: Method::POST,
      ..Default::default()
    }
  }

  pub fn patch(uri: &str) -> Self {
    Self {
      uri: uri.to_string(),
      method: Method::PATCH,
      ..Default::default()
    }
  }

  pub fn put(uri: &str) -> Self {
    Self {
      uri: uri.to_string(),
      method: Method::PUT,
      ..Default::default()
    }
  }

  pub fn delete(uri: &str) -> Self {
    Self {
      uri: uri.to_string(),
      method: Method::DELETE,
      ..Default::default()
    }
  }

  pub fn body(mut self, body: serde_json::Value) -> Self {
    self.body = Some(body);
    self
  }

  pub fn header(mut self, key: &str, value: &str) -> Self {
    self.headers.insert(key.to_string(), value.to_string());
    self
  }

  pub fn authorization(mut self, token: &str) -> Self {
    self
      .headers
      .insert("Authorization".to_string(), format!("Bearer {}", token));
    self
  }

  pub async fn send(&self, app: &Router) -> anyhow::Result<Response<Body>> {
    let mut request = AxumRequest::builder()
      .uri(self.uri.clone())
      .method(self.method.clone());

    for (key, value) in &self.headers {
      request = request.header(key, value);
    }

    let request = if let Some(body) = &self.body {
      request
        .header("Content-Type", "application/json; charset=utf-8")
        .body(Body::from(serde_json::to_string(body)?))?
    } else {
      request.body(Body::empty())?
    };

    app
      .clone()
      .oneshot(request)
      .await
      .map_err(|e| anyhow::anyhow!("Failed to send request: {}", e))
  }
}
