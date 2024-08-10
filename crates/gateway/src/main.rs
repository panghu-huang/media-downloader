use aggregation::create_aggregation_service;
use configuration::Configuration;
use gateway::Gateway;
use models::ConnectionPool;
use rpc_client::RpcClient;
use tracing_subscriber::fmt::time::ChronoLocal;

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
  tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .with_timer(ChronoLocal::new("%Y-%m-%d %H:%M:%S%.3f".to_string()))
    .init();

  let config = Configuration::new()?;
  let client = RpcClient::try_from(&config)?;
  let connection_pool = ConnectionPool::connect(&config.database.url).await?;

  let aggregation = create_aggregation_service(&config, &connection_pool, &client);

  let gateway = Gateway::new(client, config.clone());

  let gateway_addr = config
    .app
    .gateway_addr
    .parse()
    .map_err(|err| anyhow::anyhow!("Failed to parse gateway address: {}", err))?;

  let services_addr = config
    .app
    .services_addr
    .parse()
    .map_err(|err| anyhow::anyhow!("Failed to parse services address: {}", err))?;

  tokio::select! {
    _ = aggregation.serve(services_addr) => {
      log::error!("Aggregation service failed to start");
    }
    _ = gateway.serve(gateway_addr) => {
      log::error!("Gateway failed to start");
    }
    _ = tokio::signal::ctrl_c() => {
      log::info!("Ctrl-C received, shutting down");
    }
  };

  log::info!("Shutting down");

  Ok(())
}
