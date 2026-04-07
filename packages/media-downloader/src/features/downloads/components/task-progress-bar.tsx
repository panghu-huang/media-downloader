import * as React from 'react'

interface TaskProgressBarProps {
  progress: number
}

export const TaskProgressBar: React.FC<TaskProgressBarProps> = ({
  progress,
}) => {
  return (
    <div className="w-full bg-slate-200 dark:bg-slate-700 rounded-full h-2 overflow-hidden">
      <div
        className="bg-blue-500 h-full rounded-full transition-all duration-500 ease-out"
        style={{ width: `${Math.min(progress, 100)}%` }}
      />
    </div>
  )
}
