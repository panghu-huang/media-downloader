import * as React from 'react'
import { router } from '@coodev/react/router'
import { Input } from '@/components/ui/input'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
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
      <div className="flex items-center gap-2 flex-shrink-0">
        <span className="text-sm text-slate-600 dark:text-slate-400">
          Channel:
        </span>
        <Select value={currentChannel} onValueChange={handleChannelChange}>
          <SelectTrigger className="w-[180px]">
            <SelectValue placeholder="Select channel" />
          </SelectTrigger>
          <SelectContent>
            {channels.map(ch => (
              <SelectItem key={ch.id} value={ch.id}>
                {ch.name}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      </div>
      <div className="relative flex-1">
        <svg
          className="absolute left-3 top-1/2 -translate-y-1/2 h-5 w-5 text-slate-400"
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
          className="pl-10 h-12 text-base shadow-lg"
        />
      </div>
    </div>
  )
}

export { SearchInput }
