use proto::proto;

fn main() {
  let channel_service = proto! {
    package channel;
    codec crate::json_codec::JsonCodec;

    service Channel {
      rpc DownloadTvShow(crate::channel::DownloadTVShowRequest) returns (crate::channel::DownloadTVShowResponse) {}
    }
  };

  tonic_build::manual::Builder::new()
    .out_dir("src/pb")
    .compile(&[channel_service]);
}
