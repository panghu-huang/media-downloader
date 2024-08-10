use channel::ChannelService;
use configuration::Configuration;
use models::ConnectionPool;
use protocol::channel::ChannelServer;
use protocol::tonic::transport::server::Router;
use protocol::tonic::transport::Server;
use rpc_client::RpcClient;
use std::net::SocketAddr;

pub struct AggregationService {
  channel: ChannelService,
}

impl AggregationService {
  pub async fn serve(self, addr: SocketAddr) -> anyhow::Result<()> {
    log::info!("Aggregation service is listening on {}", addr);

    self
      .build_services()
      .serve(addr)
      .await
      .map_err(|err| anyhow::anyhow!("Failed to start aggregation service: {}", err))
  }

  #[cfg(feature = "testing")]
  pub async fn serve_with_incoming(
    self,
    incoming: testing::protocol::IncomingServer,
  ) -> anyhow::Result<()> {
    self.build_services().serve_with_incoming(incoming).await?;

    Ok(())
  }

  fn build_services(self) -> Router {
    Server::builder().add_service(ChannelServer::new(self.channel))
  }
}

pub fn create_aggregation_service(
  _configuration: &Configuration,
  _pool: &ConnectionPool,
  _rpc_client: &RpcClient,
) -> AggregationService {
  let channel = ChannelService::new();

  AggregationService { channel }
}
