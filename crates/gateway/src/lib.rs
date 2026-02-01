mod controllers;
#[allow(dead_code)]
mod error;
mod extracts;
mod middlewares;
mod state;

use crate::error::AppError;
use axum::http::{HeaderValue, Method};
use axum::routing::{get, post};
use axum::Router;
use configuration::Configuration;
use controllers::{channel, media};
use rpc_client::RpcClient;
use state::AppState;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

pub type Result<T> = std::result::Result<T, error::AppError>;

pub struct Gateway {
  pub client: RpcClient,
  pub config: Configuration,
}

impl Gateway {
  pub fn new(client: RpcClient, config: Configuration) -> Self {
    Self { client, config }
  }

  pub async fn serve(&self, addr: SocketAddr) -> Result<()> {
    let app = self.router();

    let listener = TcpListener::bind(addr).await.map_err(|err| {
      AppError::internal_server_error(format!("Failed to bind to address: {}", err))
    })?;

    log::info!("Gateway is listening on {}", addr);

    axum::serve(listener, app)
      .await
      .map_err(|err| AppError::internal_server_error(format!("Failed to start server: {}", err)))
  }

  pub fn router(&self) -> Router {
    let state = AppState::new(self.client.clone(), self.config.clone());

    let router = Router::new()
      .route("/channels", get(channel::get_channels))
      .route(
        "/channels/:channel_name/media/:media_id",
        get(media::get_media_metadata),
      )
      .route(
        "/channels/:channel_name/media/:media_id/playlist",
        get(media::get_media_playlist),
      )
      .route("/media/download", post(media::download_media))
      .route("/media/batch_download", post(media::batch_download_media))
      .route("/media/search", get(media::search_media))
      // Log incoming requests and responses
      .layer(axum::middleware::from_fn(middlewares::logging))
      // Add a revision to the response headers
      .layer(axum::middleware::from_fn(middlewares::revision))
      // Enable CORS
      .layer(
        CorsLayer::new()
          .allow_origin("*".parse::<HeaderValue>().unwrap())
          .allow_methods(vec![
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
          ])
          .allow_headers(Any),
      )
      .with_state(state)
      .fallback(handler_404);

    Router::new().nest("/api/v1", router)
  }
}

async fn handler_404() -> AppError {
  AppError::not_found(
    "The requested resource could not be found. Please check your request and try again.",
  )
}
