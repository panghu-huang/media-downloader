# 05 - MediaService 下载追踪（核心改动）

## 文件

`crates/services/media/src/lib.rs`

## batch_download_media() 改造

### tokio::spawn 之前：创建任务

```rust
let task_ids: Vec<TaskId> = (0..request.count)
    .map(|idx| {
        let ep = request.start_number + (idx as u32);
        self.task_manager.create_task(
            request.channel.clone(),
            request.media_id.clone(),
            metadata.name.clone(),
            Some(ep),
        )
    })
    .collect();
```

确保任务在 gRPC 返回前就创建好，前端可以立即看到。

### spawn 内部循环：追踪进度

替换现有的日志逻辑，将 `DownloadProgressItem` 事件映射到 TaskManager：

```rust
while let Some(evt) = progress.message().await.unwrap() {
    match evt {
        DownloadProgressItem::Started { total_segments_of_media, .. } => {
            task_manager.task_started(&task_id, total_segments_of_media);
        }
        DownloadProgressItem::SegmentDownloaded { .. } => {
            task_manager.task_segment_downloaded(&task_id);
        }
        DownloadProgressItem::TransformingVideo { .. } => {
            task_manager.task_transforming(&task_id);
        }
        DownloadProgressItem::Done { local_path, .. } => {
            task_manager.task_completed(&task_id);
            return Ok(PathBuf::from(local_path));
        }
        DownloadProgressItem::Failed { reason, .. } => {
            task_manager.task_failed(&task_id, &reason);
            anyhow::bail!("Download failed: {}", reason);
        }
    }
}
```

### 循环 break 时：取消剩余任务

```rust
// 如果某集失败导致 break，将剩余任务标记为 Failed
for remaining_id in &task_ids[idx+1..] {
    task_manager.task_failed(remaining_id, "Cancelled: previous episode failed");
}
```

## download_media() 单集下载

同样处理：创建单个任务，追踪进度。

## 启用 segment 进度

`crates/services/channel/src/common/download_media.rs` 第 122 行：

```rust
// 取消注释：
stream_clone.segment_downloaded(&msg);
```

使中间进度事件（每 3 秒视频时间一次）流过 gRPC stream，TaskManager 才能接收到 segment 级别更新。
