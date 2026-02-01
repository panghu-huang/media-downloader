use crate::extracts::RpcClient;
use axum::extract::Json;
use protocol::channel::GetChannelsRequest;
use protocol::channel::GetChannelsResponse;

/// Handler for `GET /api/v1/channels`
pub async fn get_channels(
  RpcClient(rpc_client): RpcClient,
) -> crate::Result<Json<GetChannelsResponse>> {
  let mut channel_client = rpc_client.channel.clone();

  let res = channel_client
    .get_channels(GetChannelsRequest {})
    .await?
    .into_inner();

  Ok(Json(res))
}
