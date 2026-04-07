# 03 - 后端串联 TaskManager

## 3a. main.rs (`crates/gateway/src/main.rs`)

创建 `Arc<TaskManager>`，同时传给 `create_aggregation_service()` 和 `Gateway::new()`：

```rust
let task_manager = TaskManager::new();

let aggregation = create_aggregation_service(&config, &client, task_manager.clone());
let gateway = Gateway::new(client, config.clone(), task_manager);
```

## 3b. state.rs (`crates/gateway/src/state.rs`)

`AppState` 新增字段：

```rust
pub struct AppState {
    pub rpc_client: RpcClient,
    pub config: Configuration,
    pub task_manager: Arc<TaskManager>,
}
```

## 3c. gateway lib.rs (`crates/gateway/src/lib.rs`)

- `Gateway` 结构体新增 `task_manager: Arc<TaskManager>` 字段
- 路由新增：
  - `.route("/downloads", get(downloads::list_downloads))`
  - `.route("/downloads/events", get(downloads::download_events_sse))`

## 3d. aggregation (`crates/services/aggregation/src/lib.rs`)

`create_aggregation_service` 增加 `task_manager: Arc<TaskManager>` 参数，透传到 `MediaService::new()`。

## 3e. media service (`crates/services/media/src/lib.rs`)

```rust
pub struct MediaService {
    media_dir: PathBuf,
    rpc_client: RpcClient,
    task_manager: Arc<TaskManager>,
}
```

更新 `new()` 接受 `task_manager` 参数。
