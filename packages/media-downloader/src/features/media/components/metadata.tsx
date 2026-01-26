import * as React from 'react'
import { AspectRatio } from '@/components/ui/aspect-ratio'
import { MediaMetadata } from '../types'

export interface MetadataProps {
  metadata: MediaMetadata
}

const Metadata: React.FC<MetadataProps> = ({ metadata }) => {
  return (
    <div className="flex flex-col md:flex-row gap-6 bg-white dark:bg-slate-900 rounded-xl p-6 shadow-lg">
      <div className="w-full md:w-64 flex-shrink-0">
        <AspectRatio ratio={5 / 7}>
          <img
            className="w-full h-full rounded-lg object-cover shadow-md"
            src={metadata.poster_url}
            alt={metadata.name}
          />
        </AspectRatio>
      </div>
      <div className="flex-1">
        <div className="flex items-center gap-3 mb-4">
          <h1 className="text-3xl font-bold text-slate-900 dark:text-slate-100">
            {metadata.name}
          </h1>
          <span className="px-3 py-1 bg-blue-100 dark:bg-blue-900 text-blue-700 dark:text-blue-300 rounded-full text-sm font-medium">
            {metadata.release_year}
          </span>
        </div>
        <p className="text-slate-600 dark:text-slate-400 leading-relaxed" dangerouslySetInnerHTML={{ __html: metadata.description }} />
      </div>
    </div>
  )
}

export { Metadata }
