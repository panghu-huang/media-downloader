import * as React from 'react'
import { LoaderContext } from '@/common/types'
import { Toggle } from '@/components/ui/toggle'
import { Separator } from '@/components/ui/separator'
import { Button } from '@/components/ui/button'
import { AspectRatio } from '@/components/ui/aspect-ratio'
import { MediaMetadata, MediaPlaylistItem } from '../types'
import { mediaAPI } from '../api'

export interface DetailsPageParams {
  channel: string
  id: string
}

export interface DetailsProps {
  metadata: MediaMetadata
  playlist: MediaPlaylistItem[]
}

const Metadata: React.FC<{ metadata: MediaMetadata }> = ({ metadata }) => {
  return (
    <div className="flex">
      <div style={{ width: 300 }} className="mr-4">
        <AspectRatio ratio={5 / 7}>
          <img
            className="w-full h-full"
            src={metadata.poster_url}
            alt={metadata.name}
          />
        </AspectRatio>
      </div>
      <div className="flex-1">
        <div className="flex items-center flex-row">
          <h1 className="text-lg mr-2">{metadata.name}</h1>
          <span>{metadata.release_year}</span>
        </div>
        <p>{metadata.description}</p>
      </div>
    </div>
  )
}

const isSelected = (index: number, start?: number, end?: number) => {
  if (start !== undefined && end !== undefined) {
    return index >= start && index <= end
  }

  return index === start || index === end
}

const Playlist: React.FC<{
  playlist: MediaPlaylistItem[]
  start?: number
  end?: number
  onToggle: (index: number) => void
}> = ({ playlist, start, end, onToggle }) => {
  return (
    <div>
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

const Details: React.FC<DetailsProps> = ({ metadata, playlist }) => {
  const [start, setStart] = React.useState<number | undefined>(undefined)
  const [end, setEnd] = React.useState<number | undefined>(undefined)

  const clearSelection = () => {
    setStart(undefined)
    setEnd(undefined)
  }

  const handleToggle = (index: number) => {
    if (start === undefined && end === undefined) {
      setStart(index)
      setEnd(undefined)
    } else if (start !== undefined && end !== undefined) {
      setStart(index)
      setEnd(undefined)
    } else {
      const [min, max] = ([start, end, index].filter(Boolean) as number[]).sort(
        (a, b) => a - b,
      )

      if (min === max) {
        clearSelection()
        return
      }

      setStart(min)
      setEnd(max)
    }
  }

  const download = async () => {
    if (start === undefined && end === undefined) {
      return
    }

    const startNumber = (start === undefined ? end : start) as number
    const count = (
      start === undefined || end === undefined ? 1 : end - start + 1
    ) as number

    try {
      await mediaAPI.batchDownload({
        channel: metadata.channel,
        media_id: metadata.id,
        start_number: startNumber,
        count,
      })

      alert('Download started')

      clearSelection()
    } catch (err) {
      console.error(err)
    }
  }

  return (
    <div className="p-8">
      <Metadata metadata={metadata} />
      <Separator className="mt-4 mb-4" />
      <Playlist
        playlist={playlist}
        start={start}
        end={end}
        onToggle={handleToggle}
      />
      <div className="mt-12">
        <Button
          disabled={start === undefined && end === undefined}
          onClick={download}
        >
          Download
        </Button>
      </div>
    </div>
  )
}

export const loader = async ({ params }: LoaderContext<DetailsPageParams>) => {
  const [metadata, playlist] = await Promise.all([
    mediaAPI.getMetadata(params.channel, params.id),
    mediaAPI.getPlaylist(params.channel, params.id),
  ])

  return {
    metadata,
    playlist: playlist.items,
  }
}

export { Details }
