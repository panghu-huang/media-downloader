import * as React from 'react'
import { AspectRatio } from '@/components/ui/aspect-ratio'
import { MediaMetadata } from '../types'

export interface MetadataProps {
  metadata: MediaMetadata
}

const Metadata: React.FC<MetadataProps> = ({ metadata }) => {
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

export { Metadata }
