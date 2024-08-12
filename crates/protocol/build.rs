use proto::proto;

fn main() {
  let channel_service = proto! {
    package channel;
    codec crate::json_codec::JsonCodec;

    service Channel {
      rpc DownloadTvShow(crate::channel::DownloadTVShowRequest) returns (stream crate::channel::DownloadProgress) {}
      rpc GetTvShowMetadata(crate::channel::GetTVShowMetadataRequest) returns (crate::channel::TVShowMetadata) {}
    }
  };

  let media_service = proto! {
    package media;
    codec crate::json_codec::JsonCodec;

    service Media {
      rpc DownloadTvShow(crate::media::DownloadTVShowRequest) returns (crate::Empty) {}
      rpc GetTvShowMetadata(crate::media::GetTVShowMetadataRequest) returns (crate::media::TVShowMetadata) {}
    }
  };

  tonic_build::manual::Builder::new()
    .out_dir("src/pb")
    .compile(&[channel_service, media_service]);
}
