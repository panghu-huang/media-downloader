use aggregation::create_aggregation_service;
use configuration::{Configuration, Environment};
use gateway::Gateway;
use models::ConnectionPool;
use rpc_client::RpcClient;
use testing::protocol::create_testing_channel;
// use tracing_subscriber::fmt::time::ChronoLocal;

#[macro_export]
macro_rules! response_to_json {
  ($response: ident) => {{
    use http_body_util::BodyExt;

    let body = $response.into_body().collect().await.unwrap().to_bytes();

    serde_json::from_slice(&body[..]).unwrap()
  }};
}

pub async fn setup_testing() -> Gateway {
  // tracing_subscriber::fmt()
  //   .with_max_level(tracing::Level::INFO)
  //   .with_timer(ChronoLocal::new("%Y-%m-%d %H:%M:%S%.3f".to_string()))
  //   .init();
  //
  let (client, server) = create_testing_channel().await;

  // Create a new configuration with the test environment.
  let config = Configuration::new_with_env(Environment::Test).unwrap();

  let client = RpcClient::new(client);

  let connection_pool = ConnectionPool::connect(&config.database.url).await.unwrap();

  let aggregation = create_aggregation_service(&config, &connection_pool, &client);

  tokio::task::spawn(async move {
    aggregation.serve_with_incoming(server).await.unwrap();
  });

  Gateway::new(client, config)
}
