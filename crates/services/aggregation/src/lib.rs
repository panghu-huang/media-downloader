use channel::ChannelService;
use configuration::Configuration;
use media::MediaService;
use models::ConnectionPool;
use protocol::channel::ChannelServer;
use protocol::media::MediaServer;
use protocol::tonic::transport::server::Router;
use protocol::tonic::transport::Server;
use rpc_client::RpcClient;
use std::net::SocketAddr;

pub struct AggregationService {
  channel: ChannelService,
  media: MediaService,
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
    Server::builder()
      .add_service(ChannelServer::new(self.channel))
      .add_service(MediaServer::new(self.media))
  }
}

pub fn create_aggregation_service(
  configuration: &Configuration,
  _pool: &ConnectionPool,
  rpc_client: &RpcClient,
) -> AggregationService {
  let channel = ChannelService::new(configuration);
  let media = MediaService::new(rpc_client);

  AggregationService { channel, media }
}
