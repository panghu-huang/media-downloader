use proto::proto;

fn main() {
  let channel_service = proto! {
    package channel;
    codec crate::json_codec::JsonCodec;

    service Channel {
      rpc DownloadMedia(crate::channel::DownloadMediaRequest) returns (stream crate::DownloadProgressItem) {}
      rpc GetMediaMetadata(crate::channel::GetMediaMetadataRequest) returns (crate::channel::MediaMetadata) {}
    }
  };

  let media_service = proto! {
    package media;
    codec crate::json_codec::JsonCodec;

    service Media {
      rpc DownloadMedia(crate::media::DownloadMediaRequest) returns (crate::Empty) {}
      rpc GetMediaMetadata(crate::media::GetMediaMetadataRequest) returns (crate::media::MediaMetadata) {}
    }
  };

  tonic_build::manual::Builder::new()
    .out_dir("src/pb")
    .compile(&[channel_service, media_service]);
}
