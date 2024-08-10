use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadTVShowRequest {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadTVShowResponse {}

mod channel_inner {
  include!("./pb/channel.Channel.rs");
}

pub use channel_inner::{
  channel_client::ChannelClient,
  channel_server::{Channel as ChannelExt, ChannelServer},
};
