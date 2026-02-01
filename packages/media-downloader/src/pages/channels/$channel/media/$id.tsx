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
import { SearchInput } from '@/components/search-input'
import { Button } from '@/components/ui/button'
import { useToast } from '@/components/ui/toast'

export interface DetailsPageParams {
  channel: string
  id: string
}

export interface DetailsProps {
  channel: string
  metadata: MediaMetadata
  playlist: MediaPlaylistItem[]
}

const Details: React.FC<DetailsProps> = ({ metadata, playlist }) => {
  const { start, end, toggle, clearSelection } = useSelection()
  const { toast } = useToast()

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

      toast(
        count > 1 ? `${count} episodes download started` : 'Download started',
        'success',
      )

      clearSelection()
    } catch (err) {
      console.error(err)
      toast('Failed to start download', 'error')
    }
  }

  const selectedCount =
    start !== undefined && end !== undefined
      ? end - start + 1
      : start !== undefined || end !== undefined
        ? 1
        : 0

  return (
    <div className="min-h-screen bg-slate-50 dark:bg-slate-950">
      <div className="sticky top-0 z-10 bg-white/80 dark:bg-slate-900/80 backdrop-blur-lg border-b border-slate-200 dark:border-slate-800">
        <div className="max-w-7xl mx-auto p-4">
          <SearchInput />
        </div>
      </div>
      <div className="max-w-7xl mx-auto p-6 space-y-6">
        <Metadata metadata={metadata} />
        <Playlist
          playlist={playlist}
          start={start}
          end={end}
          onToggle={toggle}
        />
        <div className="flex items-center justify-between bg-white dark:bg-slate-900 rounded-xl p-6 shadow-lg">
          <div>
            {selectedCount > 0 && (
              <p className="text-sm text-slate-600 dark:text-slate-400">
                {selectedCount} episode{selectedCount > 1 ? 's' : ''} selected
              </p>
            )}
          </div>
          <Button
            disabled={start === undefined && end === undefined}
            onClick={download}
            className="px-8 py-2 bg-blue-600 hover:bg-blue-700 disabled:bg-slate-300 dark:disabled:bg-slate-700 transition-colors"
          >
            {selectedCount > 0
              ? `Download ${selectedCount} Episode${selectedCount > 1 ? 's' : ''}`
              : 'Select Episodes'}
          </Button>
        </div>
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
    channel: params.channel,
    metadata,
    playlist: playlist.items,
  }
}

Details.getInitialProps = loader

export default Details
