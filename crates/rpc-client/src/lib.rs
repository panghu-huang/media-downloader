use configuration::Configuration;
use protocol::channel::ChannelClient;
use protocol::tonic::transport::{Channel as TransportChannel, Endpoint};

#[derive(Clone)]
pub struct RpcClient {
  pub channel: ChannelClient<TransportChannel>,
}

impl RpcClient {
  pub fn new(channel: TransportChannel) -> Self {
    let channel = ChannelClient::new(channel);

    Self { channel }
  }
}

impl TryFrom<&Configuration> for RpcClient {
  type Error = anyhow::Error;

  fn try_from(config: &Configuration) -> Result<Self, Self::Error> {
    let channel =
      Endpoint::try_from(format!("http://{}", config.app.services_addr))?.connect_lazy();

    Ok(Self::new(channel))
  }
}
