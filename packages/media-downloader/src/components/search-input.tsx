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
    <Input
      type="search"
      placeholder="Search"
      value={keyword}
      onChange={handleChange}
      onKeyDown={handleKeyDown}
    />
  )
}

export { SearchInput }
