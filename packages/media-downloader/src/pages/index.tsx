import * as React from 'react'
import { SearchInput } from '@/components/search-input'

const Home: React.FC = () => {
  return (
    <div className="flex items-center justify-center min-h-screen bg-gradient-to-br from-slate-50 to-slate-100 dark:from-slate-950 dark:to-slate-900">
      <div className="w-full max-w-2xl px-4">
        <div className="text-center mb-8">
          <h1 className="text-5xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent mb-3">
            Media Downloader
          </h1>
          <p className="text-slate-600 dark:text-slate-400 text-lg">
            Search and download your favorite media content
          </p>
        </div>
        <div className="relative">
          <SearchInput />
          <div className="mt-4 text-center text-sm text-slate-500 dark:text-slate-500">
            Press <kbd className="px-2 py-1 bg-slate-200 dark:bg-slate-800 rounded text-xs font-mono">Enter</kbd> to search
          </div>
        </div>
      </div>
    </div>
  )
}

export default Home
