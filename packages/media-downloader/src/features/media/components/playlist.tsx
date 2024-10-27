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
    <div className="media-playlist">
      {playlist.map(item => {
        return (
          <Toggle
            key={item.number}
            className="m-1"
            title={item.text}
            pressed={isSelected(item.number, start, end)}
            onClick={() => onToggle(item.number)}
          >
            {item.text}
          </Toggle>
        )
      })}
    </div>
  )
}

export { Playlist }
