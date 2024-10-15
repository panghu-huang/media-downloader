import * as React from 'react'
import { LoaderContext } from '@/common/types'
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

const Details: React.FC<DetailsProps> = props => {
  return (
    <div>
      <h1>Download</h1>
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
