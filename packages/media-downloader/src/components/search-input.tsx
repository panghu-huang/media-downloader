import * as React from 'react'
import { router } from '@coodev/react/router'
import { Input } from '@/components/ui/input'

export interface SearchInputProps {
  initialValue?: string
}

const SearchInput: React.FC<SearchInputProps> = ({ initialValue = '' }) => {
  const [keyword, setKeyword] = React.useState(initialValue)

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setKeyword(e.target.value)
  }

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter') {
      handleSearch()
    }
  }

  const handleSearch = () => {
    router.push('/search?q=' + encodeURIComponent(keyword) + '&page=1')
  }

  return (
    <div className="relative">
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
  )
}

export { SearchInput }
