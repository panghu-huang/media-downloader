use serde::{Deserialize, Serialize};
use stream::Stream;

pub type DownloadProgressStream = stream::Stream<DownloadProgressItem, DownloadProgressResponse>;

pub type DownloadProgressReceiver =
  stream::Receiver<DownloadProgressItem, DownloadProgressResponse>;

pub type DownloadProgressResponse = tonic::Result<DownloadProgressItem>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DownloadProgressItem {
  Started {
    total_segments_of_media: usize,
    started_at: String,
  },
  SegmentDownloaded {
    message: String,
    started_at: String,
  },
  TransformingVideo {
    started_at: String,
  },
  Done {
    local_path: String,
    completed_at: String,
  },
  Failed {
    reason: String,
    completed_at: String,
  },
}

pub trait DownloadProgressExt {
  fn start(&self, total_segments: usize);
  fn segment_downloaded(&self, msg: &str);
  fn transforming_video(&self);
  fn done(&self, local_path: &str);
  fn failed(&self, reason: &str);
}

impl DownloadProgressExt for Stream<DownloadProgressItem, DownloadProgressResponse> {
  fn start(&self, total_segments_of_media: usize) {
    self.send(DownloadProgressItem::Started {
      total_segments_of_media,
      started_at: now(),
    });
  }

  fn segment_downloaded(&self, msg: &str) {
    self.send(DownloadProgressItem::SegmentDownloaded {
      message: msg.to_string(),
      started_at: now(),
    });
  }

  fn transforming_video(&self) {
    self.send(DownloadProgressItem::TransformingVideo { started_at: now() })
  }

  fn done(&self, local_path: &str) {
    self.send(DownloadProgressItem::Done {
      local_path: local_path.to_owned(),
      completed_at: now(),
    });
    self.end();
  }

  fn failed(&self, reason: &str) {
    self.send(DownloadProgressItem::Failed {
      reason: reason.to_owned(),
      completed_at: now(),
    });
    self.end();
  }
}

fn now() -> String {
  chrono::Utc::now().to_string()
}
