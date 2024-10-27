import * as React from 'react'
import Link from '@coodev/react/link'
import { mediaAPI, type SearchMediaResponse } from '@/features/media'
import { type LoaderContext } from '@/common/types'
import { AspectRatio } from '@/components/ui/aspect-ratio'

export type SearchProps = SearchMediaResponse

const Search: React.FC<SearchProps> = ({ items }) => {
  return (
    <div className="grid grid-cols-6">
      {items.map(item => {
        return (
          <Link key={item.id} to={`/channels/${item.channel}/media/${item.id}`}>
            <div key={item.id} className="p-4">
              <AspectRatio ratio={5 / 7}>
                <img
                  src={item.poster_url}
                  alt={item.name}
                  className="w-full h-full"
                />
              </AspectRatio>
              <h3 className="mt-2 font-bold">{item.name}</h3>
            </div>
          </Link>
        )
      })}
    </div>
  )
}

const loader = ({ url }: LoaderContext): Promise<SearchProps> => {
  const search = url.slice(url.indexOf('?'))
  const params = new URLSearchParams(search)

  const keyword = params.get('q')
  const page = +(params.get('page') || 1)

  if (!keyword) {
    throw new Error('Keyword is required')
  }

  return mediaAPI.search({ keyword, page })
}

Search.getInitialProps = loader

export default Search
