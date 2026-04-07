import * as React from 'react'
import { DownloadTask } from '../types'
import { TaskStatusBadge } from './task-status-badge'
import { TaskProgressBar } from './task-progress-bar'

interface TaskCardProps {
  task: DownloadTask
}

function formatTime(dateStr: string) {
  const date = new Date(dateStr)
  const h = String(date.getHours()).padStart(2, '0')
  const m = String(date.getMinutes()).padStart(2, '0')
  const s = String(date.getSeconds()).padStart(2, '0')
  return `${h}:${m}:${s}`
}

export const TaskCard: React.FC<TaskCardProps> = ({ task }) => {
  const episodeLabel = task.episode_number
    ? ` - Episode ${task.episode_number}`
    : ''

  return (
    <div className="bg-white dark:bg-slate-900 rounded-lg border border-slate-200 dark:border-slate-800 p-4">
      <div className="flex items-start justify-between gap-4">
        <div className="flex-1 min-w-0">
          <h3 className="font-semibold text-slate-900 dark:text-slate-100 truncate">
            {task.media_name}
            {episodeLabel}
          </h3>
          <p className="text-xs text-slate-500 dark:text-slate-400 mt-1">
            {task.channel} / {task.media_id}
          </p>
        </div>
        <div className="flex items-center gap-3 shrink-0">
          <span className="text-xs text-slate-400 dark:text-slate-500">
            {formatTime(task.updated_at)}
          </span>
          <TaskStatusBadge status={task.status} />
        </div>
      </div>

      {task.status === 'Downloading' && (
        <div className="mt-3">
          <div className="flex items-center justify-between mb-1">
            <span className="text-xs text-slate-500 dark:text-slate-400">
              {task.downloaded_segments} / {task.total_segments ?? '?'} segments
            </span>
            <span className="text-xs font-medium text-blue-600 dark:text-blue-400">
              {task.progress}%
            </span>
          </div>
          <TaskProgressBar progress={task.progress} />
        </div>
      )}

      {task.status === 'Failed' && task.error_message && (
        <p className="mt-2 text-xs text-red-600 dark:text-red-400 bg-red-50 dark:bg-red-900/20 rounded px-2 py-1">
          {task.error_message}
        </p>
      )}
    </div>
  )
}
