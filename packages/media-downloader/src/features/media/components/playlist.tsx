import * as React from 'react'
import { Toggle } from '@/components/ui/toggle'
import { MediaPlaylistItem } from '../types'

export interface PlaylistProps {
  playlist: MediaPlaylistItem[]
  start?: number
  end?: number
  onToggle: (index: number) => void
}

const isSelected = (index: number, start?: number, end?: number) => {
  if (start !== undefined && end !== undefined) {
    return index >= start && index <= end
  }

  return index === start || index === end
}

const Playlist: React.FC<PlaylistProps> = ({
  playlist,
  start,
  end,
  onToggle,
}) => {
  return (
    <div className="bg-white dark:bg-slate-900 rounded-xl p-6 shadow-lg">
      <div className="mb-4 flex items-center justify-between">
        <h2 className="text-xl font-semibold text-slate-900 dark:text-slate-100">
          Episodes
        </h2>
        <span className="text-sm text-slate-500 dark:text-slate-400">
          {playlist.length} episodes
        </span>
      </div>
      <div className="flex flex-wrap gap-2">
        {playlist.map((item, index) => {
          const prefix = String(index + 1)
          const text = String(item.text).trim()

          return (
            <Toggle
              key={item.number}
              title={item.text}
              pressed={isSelected(item.number, start, end)}
              onClick={() => onToggle(item.number)}
              className="data-[state=on]:bg-blue-600 data-[state=on]:text-white hover:bg-slate-100 dark:hover:bg-slate-800 transition-colors"
            >
              {prefix === text ? text : `${prefix} (${text})`}
            </Toggle>
          )
        })}
      </div>
    </div>
  )
}

export { Playlist }
