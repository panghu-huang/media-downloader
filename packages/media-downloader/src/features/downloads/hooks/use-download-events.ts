import { useState, useEffect, useMemo } from 'react'
import { getRuntimeConfig } from '@coodev/react/config'
import { RuntimeConfig } from '@/common/types'
import { DownloadTask, TaskEvent } from '../types'

export function useDownloadEvents(
  initialTasks: DownloadTask[],
): DownloadTask[] {
  const [taskMap, setTaskMap] = useState<Map<string, DownloadTask>>(() => {
    const map = new Map<string, DownloadTask>()
    for (const task of initialTasks) {
      map.set(task.id, task)
    }
    return map
  })

  useEffect(() => {
    if (typeof window === 'undefined') return

    const runtimeConfig = getRuntimeConfig() as RuntimeConfig
    const baseUrl = runtimeConfig.apiBaseURL

    const eventSource = new EventSource(`${baseUrl}/downloads/events`)

    eventSource.addEventListener('init', (e: MessageEvent) => {
      try {
        const tasks: DownloadTask[] = JSON.parse(e.data)
        setTaskMap(() => {
          const map = new Map<string, DownloadTask>()
          for (const task of tasks) {
            map.set(task.id, task)
          }
          return map
        })
      } catch {
        // ignore parse errors
      }
    })

    eventSource.addEventListener('task_update', (e: MessageEvent) => {
      try {
        const event: TaskEvent = JSON.parse(e.data)
        setTaskMap((prev) => {
          const next = new Map(prev)
          next.set(event.task_id, event.task)
          return next
        })
      } catch {
        // ignore parse errors
      }
    })

    return () => {
      eventSource.close()
    }
  }, [])

  const tasks = useMemo(() => {
    const list = Array.from(taskMap.values())
    list.sort(
      (a, b) =>
        new Date(b.created_at).getTime() - new Date(a.created_at).getTime(),
    )
    return list
  }, [taskMap])

  return tasks
}
