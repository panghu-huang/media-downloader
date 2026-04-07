import * as React from 'react'
import { type LoaderContext } from '@/common/types'
import { SearchInput } from '@/components/search-input'
import {
  type DownloadTask,
  type TaskStatus,
  downloadsAPI,
  useDownloadEvents,
  TaskCard,
} from '@/features/downloads'

export interface DownloadsProps {
  tasks: DownloadTask[]
}

const activeStatuses: TaskStatus[] = ['Pending', 'Downloading', 'Transforming']

const Downloads: React.FC<DownloadsProps> = ({ tasks: initialTasks }) => {
  const tasks = useDownloadEvents(initialTasks)

  const activeTasks = tasks.filter((t) => activeStatuses.includes(t.status))
  const recentTasks = tasks.filter((t) => !activeStatuses.includes(t.status))

  return (
    <div className="min-h-screen bg-slate-50 dark:bg-slate-950">
      <div className="sticky top-0 z-10 bg-white/80 dark:bg-slate-900/80 backdrop-blur-lg border-b border-slate-200 dark:border-slate-800">
        <div className="max-w-7xl mx-auto p-4">
          <SearchInput />
        </div>
      </div>
      <div className="max-w-7xl mx-auto p-6">
        <h2 className="text-2xl font-bold text-slate-900 dark:text-slate-100 mb-6">
          Downloads
        </h2>

        {tasks.length === 0 && (
          <div className="text-center py-20">
            <p className="text-slate-500 dark:text-slate-400">
              No download tasks yet.
            </p>
          </div>
        )}

        {activeTasks.length > 0 && (
          <section className="mb-8">
            <h3 className="text-sm font-medium text-slate-500 dark:text-slate-400 uppercase tracking-wider mb-3">
              Active ({activeTasks.length})
            </h3>
            <div className="space-y-3">
              {activeTasks.map((task) => (
                <TaskCard key={task.id} task={task} />
              ))}
            </div>
          </section>
        )}

        {recentTasks.length > 0 && (
          <section>
            <h3 className="text-sm font-medium text-slate-500 dark:text-slate-400 uppercase tracking-wider mb-3">
              Recent ({recentTasks.length})
            </h3>
            <div className="space-y-3">
              {recentTasks.map((task) => (
                <TaskCard key={task.id} task={task} />
              ))}
            </div>
          </section>
        )}
      </div>
    </div>
  )
}

const loader = async (_ctx: LoaderContext): Promise<DownloadsProps> => {
  const tasks = await downloadsAPI.listDownloads()

  return { tasks }
}

Downloads.getInitialProps = loader

export default Downloads
