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
  InProgress {
    message: String,
    started_at: String,
  },
  Done {
    completed_at: String,
  },
}

pub trait DownloadProgressExt {
  fn start(&self, total_segments: usize);
  fn in_progress(&self, msg: &str);
  fn done(&self);
}

impl DownloadProgressExt for Stream<DownloadProgressItem, DownloadProgressResponse> {
  fn start(&self, total_segments_of_media: usize) {
    self.send(DownloadProgressItem::Started {
      total_segments_of_media,
      started_at: now(),
    });
  }

  fn in_progress(&self, msg: &str) {
    self.send(DownloadProgressItem::InProgress {
      message: msg.to_string(),
      started_at: now(),
    });
  }

  fn done(&self) {
    self.send(DownloadProgressItem::Done {
      completed_at: now(),
    });
    self.end();
  }
}

fn now() -> String {
  chrono::Utc::now().to_string()
}
