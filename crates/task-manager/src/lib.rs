use chrono::{DateTime, Utc};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::broadcast;

pub type TaskId = String;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TaskStatus {
  Pending,
  Downloading,
  Transforming,
  Completed,
  Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadTask {
  pub id: TaskId,
  pub channel: String,
  pub media_id: String,
  pub media_name: String,
  pub episode_number: Option<u32>,
  pub status: TaskStatus,
  pub progress: u8,
  pub total_segments: Option<usize>,
  pub downloaded_segments: usize,
  pub error_message: Option<String>,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskEvent {
  pub task_id: TaskId,
  pub task: DownloadTask,
}

pub struct TaskManager {
  tasks: RwLock<HashMap<TaskId, DownloadTask>>,
  sender: broadcast::Sender<TaskEvent>,
}

impl TaskManager {
  pub fn new() -> Arc<Self> {
    let (sender, _) = broadcast::channel(256);
    Arc::new(Self {
      tasks: RwLock::new(HashMap::new()),
      sender,
    })
  }

  pub fn create_task(
    &self,
    channel: String,
    media_id: String,
    media_name: String,
    episode_number: Option<u32>,
  ) -> TaskId {
    let id = uuid::Uuid::new_v4().to_string();
    let now = Utc::now();

    let task = DownloadTask {
      id: id.clone(),
      channel,
      media_id,
      media_name,
      episode_number,
      status: TaskStatus::Pending,
      progress: 0,
      total_segments: None,
      downloaded_segments: 0,
      error_message: None,
      created_at: now,
      updated_at: now,
    };

    let mut tasks = self.tasks.write();
    tasks.insert(id.clone(), task.clone());
    drop(tasks);

    self.broadcast(TaskEvent {
      task_id: id.clone(),
      task,
    });

    id
  }

  pub fn task_started(&self, task_id: &str, total_segments: usize) {
    self.update_task(task_id, |task| {
      task.status = TaskStatus::Downloading;
      task.total_segments = Some(total_segments);
    });
  }

  pub fn task_segment_downloaded(&self, task_id: &str) {
    self.update_task(task_id, |task| {
      task.downloaded_segments += 1;
      if let Some(total) = task.total_segments {
        if total > 0 {
          task.progress = ((task.downloaded_segments as f64 / total as f64) * 100.0).min(99.0) as u8;
        }
      }
    });
  }

  pub fn task_transforming(&self, task_id: &str) {
    self.update_task(task_id, |task| {
      task.status = TaskStatus::Transforming;
      task.progress = 99;
    });
  }

  pub fn task_completed(&self, task_id: &str) {
    self.update_task(task_id, |task| {
      task.status = TaskStatus::Completed;
      task.progress = 100;
    });
  }

  pub fn task_failed(&self, task_id: &str, reason: &str) {
    self.update_task(task_id, |task| {
      task.status = TaskStatus::Failed;
      task.error_message = Some(reason.to_string());
    });
  }

  pub fn list_tasks(&self) -> Vec<DownloadTask> {
    self.cleanup_old_tasks();

    let tasks = self.tasks.read();
    let mut list: Vec<DownloadTask> = tasks.values().cloned().collect();
    list.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    list
  }

  pub fn subscribe(&self) -> broadcast::Receiver<TaskEvent> {
    self.sender.subscribe()
  }

  fn update_task<F>(&self, task_id: &str, updater: F)
  where
    F: FnOnce(&mut DownloadTask),
  {
    let mut tasks = self.tasks.write();
    if let Some(task) = tasks.get_mut(task_id) {
      updater(task);
      task.updated_at = Utc::now();
      let event = TaskEvent {
        task_id: task_id.to_string(),
        task: task.clone(),
      };
      drop(tasks);
      self.broadcast(event);
    }
  }

  fn broadcast(&self, event: TaskEvent) {
    // Ignore send errors (no active receivers)
    let _ = self.sender.send(event);
  }

  fn cleanup_old_tasks(&self) {
    let now = Utc::now();
    let mut tasks = self.tasks.write();
    tasks.retain(|_, task| {
      if task.status == TaskStatus::Completed || task.status == TaskStatus::Failed {
        let age = now.signed_duration_since(task.updated_at);
        age.num_minutes() < 60
      } else {
        true
      }
    });
  }
}
