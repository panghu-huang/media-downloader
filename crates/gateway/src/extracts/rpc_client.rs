use crate::AppState;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use std::convert::Infallible;

pub struct RpcClient(pub rpc_client::RpcClient);

#[axum::async_trait]
impl FromRequestParts<AppState> for RpcClient {
  type Rejection = Infallible;

  async fn from_request_parts(
    _parts: &mut Parts,
    state: &AppState,
  ) -> Result<Self, Self::Rejection> {
    Ok(Self(state.rpc_client.clone()))
  }
}
