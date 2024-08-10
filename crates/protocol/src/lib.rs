mod json_codec;

pub mod channel;

pub mod tonic {
  pub use ::tonic::*;
}

use serde::{Deserialize, Serialize};
use std::pin::Pin;
use tokio_stream::Stream;
use tonic::Status;

pub type StreamResponse<T> = Pin<Box<dyn Stream<Item = Result<T, Status>> + Send>>;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Empty {}
