import * as React from 'react'
import { type LoaderContext } from '@/common/types'
import {
  type MediaMetadata,
  type MediaPlaylistItem,
  Metadata,
  Playlist,
  mediaAPI,
} from '@/features/media'
import { useSelection } from '@/hooks/use-selection'
import { Separator } from '@/components/ui/separator'
import { Button } from '@/components/ui/button'

export interface DetailsPageParams {
  channel: string
  id: string
}

export interface DetailsProps {
  metadata: MediaMetadata
  playlist: MediaPlaylistItem[]
}

const Details: React.FC<DetailsProps> = ({ metadata, playlist }) => {
  const { start, end, toggle, clearSelection } = useSelection()

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
      <Playlist playlist={playlist} start={start} end={end} onToggle={toggle} />
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

const loader = async ({
  params,
}: LoaderContext<DetailsPageParams>): Promise<DetailsProps> => {
  const [metadata, playlist] = await Promise.all([
    mediaAPI.getMetadata(params.channel, params.id),
    mediaAPI.getPlaylist(params.channel, params.id),
  ])

  return {
    metadata,
    playlist: playlist.items,
  }
}

Details.getInitialProps = loader

export default Details
