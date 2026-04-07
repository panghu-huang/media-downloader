import * as React from 'react'
import { cn } from '@/common/utils'
import { TaskStatus } from '../types'

const statusConfig: Record<
  TaskStatus,
  { label: string; className: string }
> = {
  Pending: {
    label: 'Pending',
    className: 'bg-slate-100 text-slate-600 dark:bg-slate-800 dark:text-slate-400',
  },
  Downloading: {
    label: 'Downloading',
    className:
      'bg-blue-100 text-blue-700 dark:bg-blue-900/50 dark:text-blue-400 animate-pulse',
  },
  Transforming: {
    label: 'Transforming',
    className:
      'bg-amber-100 text-amber-700 dark:bg-amber-900/50 dark:text-amber-400',
  },
  Completed: {
    label: 'Completed',
    className:
      'bg-green-100 text-green-700 dark:bg-green-900/50 dark:text-green-400',
  },
  Failed: {
    label: 'Failed',
    className: 'bg-red-100 text-red-700 dark:bg-red-900/50 dark:text-red-400',
  },
}

interface TaskStatusBadgeProps {
  status: TaskStatus
}

export const TaskStatusBadge: React.FC<TaskStatusBadgeProps> = ({ status }) => {
  const config = statusConfig[status]

  return (
    <span
      className={cn(
        'inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium',
        config.className,
      )}
    >
      {config.label}
    </span>
  )
}
