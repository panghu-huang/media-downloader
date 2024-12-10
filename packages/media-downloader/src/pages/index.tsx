import * as React from 'react'
import { SearchInput } from '@/components/search-input'
import { Label } from '@/components/ui/label'

const Home: React.FC = () => {
  return (
    <div className="flex items-center justify-center h-screen">
      <div className="grid w-full max-w-sm items-center gap-1.5">
        <Label htmlFor="search">Search</Label>
        <SearchInput />
      </div>
    </div>
  )
}

export default Home
