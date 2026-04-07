export type TaskStatus =
  | 'Pending'
  | 'Downloading'
  | 'Transforming'
  | 'Completed'
  | 'Failed'

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
