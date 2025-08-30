use configuration::Configuration;
use rpc_client::RpcClient;

#[derive(Clone)]
pub struct AppState {
  pub rpc_client: RpcClient,
  #[allow(dead_code)]
  pub config: Configuration,
}

impl AppState {
  pub fn new(rpc_client: RpcClient, config: Configuration) -> Self {
    Self { rpc_client, config }
  }
}
