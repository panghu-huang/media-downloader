# 06 - 前端实现

## 目录结构

```
packages/media-downloader/src/
  features/downloads/
    types.ts                           -- 类型定义
    api/
      api.ts                           -- DownloadsAPI 类
      index.ts                         -- 实例化
    hooks/
      use-download-events.ts           -- SSE Hook
    components/
      task-card.tsx                     -- 任务卡片
      task-status-badge.tsx            -- 状态标签
      task-progress-bar.tsx            -- 进度条
      index.ts                         -- 导出
    index.ts                           -- 功能模块导出
  pages/
    downloads.tsx                      -- 下载页面
```

## 类型定义 (types.ts)

```typescript
export type TaskStatus = 'Pending' | 'Downloading' | 'Transforming' | 'Completed' | 'Failed'

export interface DownloadTask {
  id: string
  channel: string
  media_id: string
  media_name: string
  episode_number: number | null
  status: TaskStatus
  progress: number
  total_segments: number | null
  downloaded_segments: number
  error_message: string | null
  created_at: string
  updated_at: string
}

export interface TaskEvent {
  task_id: string
  task: DownloadTask
}
```

## SSE Hook (use-download-events.ts)

- `useEffect` 中创建 `EventSource` 连接 `/api/v1/downloads/events`
- `init` 事件：设置完整任务列表到 `Map<string, DownloadTask>`
- `task_update` 事件：upsert 单个任务
- `useMemo` 导出排序数组（最新的在前）
- unmount 时 `eventSource.close()`
- 仅客户端运行，SSR 时不连接

## 下载页面 (downloads.tsx)

- SSR loader：通过 `GET /api/v1/downloads` 获取初始任务列表
- 客户端 hydration 后用 `useDownloadEvents()` 实时更新
- 分两组展示：
  - **进行中**（Pending / Downloading / Transforming）
  - **已完成**（Completed / Failed）
- 沿用现有页面布局（sticky 搜索栏 + max-w-7xl 容器）
- 无任务时显示空状态

## 组件

### TaskStatusBadge

| 状态 | 样式 |
|------|------|
| Pending | `bg-slate-100 text-slate-600` |
| Downloading | `bg-blue-100 text-blue-700` + pulse 动画 |
| Transforming | `bg-amber-100 text-amber-700` |
| Completed | `bg-green-100 text-green-700` |
| Failed | `bg-red-100 text-red-700` |

### TaskProgressBar

- 仅 Downloading 状态显示
- 宽度基于 `task.progress`
- CSS transition 平滑动画
- 蓝色主题

### TaskCard

- 左侧：媒体名称 + 集数（如 "某剧名 - 第3集"）
- 中间：进度条或状态文字
- 右侧：状态标签 + 时间
- Failed 显示错误信息

## 导航入口

- 媒体详情页点击下载后，toast 提示中加入 "查看下载" 链接跳转到 `/downloads`
- SearchInput 旁增加一个下载图标/链接

## 验证

1. `GET /api/v1/downloads` 返回 `[]`
2. `GET /api/v1/downloads/events` 返回 SSE 流，初始 init 事件为空列表
3. 触发下载后 REST/SSE 均有数据
4. `/downloads` 页面显示任务卡片，进度条实时更新
5. 任务状态转换：Pending -> Downloading (带百分比) -> Transforming -> Completed
6. 边界情况：多个并发批量下载、SSE 断线重连、失败任务显示错误信息
