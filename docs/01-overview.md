# 01 - 下载任务跟踪与进度监控：总览

## 背景

当前下载是 fire-and-forget 模式：`POST /api/v1/media/batch_download` 立即返回 201，前端无法查看正在进行的下载或进度。

## 目标

1. 接口支持获取下载中的任务
2. 支持 SSE 获取下载进度
3. 提供 Web 页面查看任务和进度

## 架构决策

**关键洞察**：Gateway (Axum) 和 AggregationService (Tonic gRPC，包含 MediaService) 运行在同一个进程中 (`crates/gateway/src/main.rs`)，因此可以直接共享 `Arc<TaskManager>`，无需新增 gRPC streaming 方法。

**选择内存管理而非数据库**：作为个人媒体下载器，使用内存中的 TaskManager 足够，避免引入 PostgreSQL 依赖。已完成/失败的任务在 60 分钟后自动清理。

## 数据流

```
前端 /downloads 页面
    |
    +-- SSR: GET /api/v1/downloads (REST，获取初始任务列表)
    |
    +-- 客户端: GET /api/v1/downloads/events (SSE，实时推送)
            |
            v
    Gateway (Axum)
            |
            +-- Arc<TaskManager> (内存共享，无需 gRPC)
                    |
                    v
    MediaService (batch_download_media / download_media)
            |
            +-- 创建任务 (Pending)
            +-- 消费 ChannelService 的 DownloadProgressItem 流
            |       Started -> Downloading
            |       SegmentDownloaded -> progress++
            |       TransformingVideo -> Transforming
            |       Done -> Completed
            |       Failed -> Failed
            +-- broadcast::Sender 推送 TaskEvent -> SSE 订阅者
```

## 依赖关系

```
gateway ----------+
                  +---> task-manager
media-service ----+
```

## 任务粒度

每个批量下载的每一集是一个独立任务。例如下载第 1-5 集：
- 立即创建 5 个 Pending 任务（tokio::spawn 之前）
- 按顺序逐集下载，前端显示 1 个 Downloading + 4 个 Pending
- 若第 3 集失败，第 4、5 集标记为 Failed（"Cancelled"）
