import * as React from 'react'
import { router } from '@coodev/react/router'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'

const Home: React.FC = () => {
  const [keyword, setKeyword] = React.useState('')

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setKeyword(e.target.value)
  }

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter') {
      handleSearch()
    }
  }

  const handleSearch = () => {
    router.push('/search?q=' + keyword + '&page=1')
  }

  return (
    <div className="flex items-center justify-center h-screen">
      <div className="grid w-full max-w-sm items-center gap-1.5">
        <Label htmlFor="search">Search</Label>
        <Input
          type="search"
          placeholder="Search"
          onChange={handleChange}
          onKeyDown={handleKeyDown}
        />
      </div>
    </div>
  )
}

export default Home
