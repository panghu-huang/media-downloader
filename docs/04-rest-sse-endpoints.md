# 04 - 新增 REST + SSE 接口

## 新增文件

`crates/gateway/src/controllers/downloads.rs`

## GET /api/v1/downloads

返回当前所有下载任务列表。

```rust
pub async fn list_downloads(
    State(state): State<AppState>,
) -> Json<Vec<DownloadTask>> {
    Json(state.task_manager.list_tasks())
}
```

## GET /api/v1/downloads/events (SSE)

Server-Sent Events 实时推送下载进度。

### 事件类型

| 事件名 | 时机 | 数据 |
|--------|------|------|
| `init` | 连接时发送一次 | 完整任务列表 JSON |
| `task_update` | 任务状态/进度变化 | 单个 TaskEvent JSON |

### 实现要点

1. 调用 `task_manager.list_tasks()` 获取当前快照作为 init 事件
2. 调用 `task_manager.subscribe()` 获取 broadcast::Receiver
3. 用 `BroadcastStream` 包装 receiver，转为 Stream
4. 用 `stream::once(init).chain(updates)` 合并初始事件和后续更新
5. 返回 `Sse<impl Stream>` + 15 秒 keep-alive
6. `filter_map(|r| r.ok())` 丢弃 lagged 消息

### 依赖

- `axum::response::sse::{Event, Sse, KeepAlive}`（axum 0.7.5 自带）
- `tokio-stream` 的 `sync` feature（BroadcastStream）
- `futures` 的 `stream::once` 和 `.chain()`

### CORS

现有 CORS 配置已允许 `*` origin + GET 方法，无需额外改动。EventSource 使用 GET 请求，`text/event-stream` 不需要额外 CORS 头。
