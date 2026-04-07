# 02 - 创建 task-manager Crate

## 新增文件

- `crates/task-manager/Cargo.toml`
- `crates/task-manager/src/lib.rs`

## 核心类型

```rust
pub type TaskId = String; // UUID v4

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub progress: u8,              // 0-100
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
```

## TaskManager

```rust
pub struct TaskManager {
    tasks: RwLock<HashMap<TaskId, DownloadTask>>,
    sender: broadcast::Sender<TaskEvent>,
}
```

### 方法

| 方法 | 说明 |
|------|------|
| `new() -> Arc<Self>` | 创建实例，broadcast buffer 256 |
| `create_task(channel, media_id, media_name, episode_number) -> TaskId` | 新建 Pending 任务 |
| `task_started(id, total_segments)` | 状态 -> Downloading |
| `task_segment_downloaded(id)` | downloaded_segments++，重算 progress |
| `task_transforming(id)` | 状态 -> Transforming, progress=99 |
| `task_completed(id)` | 状态 -> Completed, progress=100 |
| `task_failed(id, reason)` | 状态 -> Failed |
| `list_tasks() -> Vec<DownloadTask>` | 返回所有任务，自动清理 60 分钟前的已完成/失败任务 |
| `subscribe() -> broadcast::Receiver<TaskEvent>` | SSE 订阅 |

## 依赖

- `serde` + `serde_json`（序列化）
- `chrono`（时间）
- `tokio` sync feature（broadcast channel）
- `parking_lot`（RwLock）
- `uuid` v4（任务 ID 生成）

## Workspace 注册

- 根 `Cargo.toml` members 加入 `"crates/task-manager"`
- workspace.dependencies 加入 `task-manager`, `uuid`
- `crates/gateway`、`crates/services/aggregation`、`crates/services/media` 各加 `task-manager` 依赖
