use protocol::tonic::transport::{Channel, Endpoint, Uri};
use tokio::io::DuplexStream;

pub type IncomingServer = tokio_stream::Once<Result<DuplexStream, std::io::Error>>;

pub async fn create_testing_channel() -> (Channel, IncomingServer) {
  let (client, server) = tokio::io::duplex(1024);

  let mut client = Some(client);

  let channel = Endpoint::try_from("http://[::]:50051")
    .unwrap()
    .connect_with_connector(tower::service_fn(move |_: Uri| {
      let client = client.take();

      async move {
        if let Some(client) = client {
          Ok(client)
        } else {
          Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Client already taken",
          ))
        }
      }
    }))
    .await
    .unwrap();

  let server = tokio_stream::once(Ok::<_, std::io::Error>(server));

  (channel, server)
}
