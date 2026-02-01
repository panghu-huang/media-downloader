import * as React from 'react'
import Link from '@coodev/react/link'
import { SearchInput } from '@/components/search-input'
import { mediaAPI, MediaMetadata } from '@/features/media'
import { type LoaderContext } from '@/common/types'
import { AspectRatio } from '@/components/ui/aspect-ratio'

export interface SearchProps {
  keyword: string
  items: MediaMetadata[]
}

const Search: React.FC<SearchProps> = ({ items, keyword }) => {
  return (
    <div className="flex flex-col min-h-screen bg-slate-50 dark:bg-slate-950">
      <div className="sticky top-0 z-10 bg-white/80 dark:bg-slate-900/80 backdrop-blur-lg border-b border-slate-200 dark:border-slate-800">
        <div className="max-w-7xl mx-auto p-4">
          <SearchInput initialValue={keyword} />
        </div>
      </div>
      <div className="max-w-7xl mx-auto w-full p-6">
        <div className="mb-6">
          <h2 className="text-2xl font-bold text-slate-900 dark:text-slate-100">
            Search Results
          </h2>
          <p className="text-slate-600 dark:text-slate-400 mt-1">
            Found {items.length} results for "{keyword}"
          </p>
        </div>
        <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-6">
          {items.map(item => {
            return (
              <Link
                key={item.id}
                to={`/channels/${item.channel}/media/${item.id}`}
              >
                <div className="group cursor-pointer">
                  <AspectRatio ratio={5 / 7}>
                    <img
                      src={item.poster_url}
                      alt={item.name}
                      className="w-full h-full rounded-lg object-cover shadow-md group-hover:shadow-xl transition-all duration-300 group-hover:scale-105"
                    />
                  </AspectRatio>
                  <h3 className="mt-3 font-semibold text-slate-900 dark:text-slate-100 line-clamp-2 group-hover:text-blue-600 dark:group-hover:text-blue-400 transition-colors">
                    {item.name}
                  </h3>
                </div>
              </Link>
            )
          })}
        </div>
      </div>
    </div>
  )
}

const loader = async ({ url }: LoaderContext): Promise<SearchProps> => {
  const search = url.slice(url.indexOf('?'))
  const params = new URLSearchParams(search)

  const q = params.get('q')
  const channel = params.get('channel')
  const page = +(params.get('page') || 1)

  if (!q) {
    throw new Error('Keyword is required')
  }
  const keyword = decodeURIComponent(q)

  const { items } = await mediaAPI.search({
    keyword,
    channel: channel || undefined,
    page,
  })

  return {
    keyword,
    items,
  }
}

Search.getInitialProps = loader

export default Search
