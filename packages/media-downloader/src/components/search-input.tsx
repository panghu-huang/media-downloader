import * as React from 'react'
import Link from '@coodev/react/link'
import { router } from '@coodev/react/router'
import { Input } from '@/components/ui/input'
import { channelAPI, Channel } from '@/features/channel'

export interface SearchInputProps {
  channel?: string
  initialValue?: string
}

const SearchInput: React.FC<SearchInputProps> = ({
  initialValue = '',
  channel = '',
}) => {
  const [keyword, setKeyword] = React.useState(initialValue)
  const [channels, setChannels] = React.useState<Channel[]>([])
  const [currentChannel, setCurrentChannel] = React.useState<string>(channel)

  React.useEffect(() => {
    // Load channels
    channelAPI.getChannels().then(({ channels }) => {
      const defaultChannel =
        channels.find(ch => ch.default)?.id || channels[0]?.id

      // Get current channel from URL
      const params = new URLSearchParams(window.location.search)
      const channelParam = channel || params.get('channel') || defaultChannel

      setCurrentChannel(channelParam)
      setChannels(channels)
    })
  }, [])

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setKeyword(e.target.value)
  }

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter') {
      handleSearch()
    }
  }

  const handleSearch = () => {
    const params = new URLSearchParams()
    params.set('q', encodeURIComponent(keyword))
    params.set('page', '1')
    if (currentChannel) {
      params.set('channel', currentChannel)
    }
    router.push('/search?' + params.toString())
  }

  const handleChannelChange = (value: string) => {
    setCurrentChannel(value)
    const params = new URLSearchParams(window.location.search)
    params.set('channel', value)
    router.push(`${window.location.pathname}?${params.toString()}`)
  }

  return (
    <div className="flex items-center gap-3">
    <div className="relative flex items-center h-12 flex-1 bg-white dark:bg-slate-950 rounded-lg shadow border border-slate-200 dark:border-slate-800 overflow-hidden">
      <div className="flex items-center border-r border-slate-200 dark:border-slate-800 pr-3">
        <select
          value={currentChannel}
          onChange={(e) => handleChannelChange(e.target.value)}
          className="h-12 w-[140px] pl-3 bg-transparent border-0 text-sm focus:outline-none focus:ring-0 cursor-pointer appearance-none"
        >
          {channels.map(ch => (
            <option key={ch.id} value={ch.id}>
              {ch.name}
            </option>
          ))}
        </select>
        <svg
          className="h-4 w-4 text-slate-400 pointer-events-none"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth={2}
            d="M19 9l-7 7-7-7"
          />
        </svg>
      </div>
      <div className="relative flex-1 flex items-center">
        <svg
          className="absolute left-3 h-5 w-5 text-slate-400"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth={2}
            d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
          />
        </svg>
        <Input
          type="search"
          placeholder="Search for movies, TV shows..."
          value={keyword}
          onChange={handleChange}
          onKeyDown={handleKeyDown}
          className="pl-10 h-full border-0 shadow-none focus-visible:ring-0 focus-visible:ring-offset-0 text-base bg-transparent"
        />
      </div>
    </div>
    <Link to="/downloads">
      <div className="flex items-center justify-center h-12 w-12 rounded-lg border border-slate-200 dark:border-slate-800 bg-white dark:bg-slate-950 shadow hover:bg-slate-50 dark:hover:bg-slate-900 transition-colors cursor-pointer" title="Downloads">
        <svg className="h-5 w-5 text-slate-600 dark:text-slate-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
        </svg>
      </div>
    </Link>
    </div>
  )
}

export { SearchInput }
