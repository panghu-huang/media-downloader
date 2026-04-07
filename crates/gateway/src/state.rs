use configuration::Configuration;
use rpc_client::RpcClient;
use std::sync::Arc;
use task_manager::TaskManager;

#[derive(Clone)]
pub struct AppState {
  pub rpc_client: RpcClient,
  #[allow(dead_code)]
  pub config: Configuration,
  pub task_manager: Arc<TaskManager>,
}

impl AppState {
  pub fn new(rpc_client: RpcClient, config: Configuration, task_manager: Arc<TaskManager>) -> Self {
    Self {
      rpc_client,
      config,
      task_manager,
    }
  }
}
