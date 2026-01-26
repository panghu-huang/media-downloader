import * as React from 'react'

export interface ErrorProps {
  isError: boolean
  error: {
    message: string
    stack: string
  }
}

const Error: React.FC<ErrorProps> = ({ error }) => {
  const [showDetails, setShowDetails] = React.useState(false)

  return (
    <div className="min-h-screen bg-gradient-to-br from-red-50 to-orange-50 dark:from-slate-950 dark:to-red-950 flex items-center justify-center p-4">
      <div className="max-w-2xl w-full">
        {/* Error Icon */}
        <div className="flex justify-center mb-6">
          <div className="bg-red-100 dark:bg-red-900/30 rounded-full p-4">
            <svg
              className="w-16 h-16 text-red-600 dark:text-red-400"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
              />
            </svg>
          </div>
        </div>

        {/* Error Title */}
        <h1 className="text-3xl font-bold text-center text-slate-900 dark:text-slate-100 mb-4">
          Oops! Something went wrong
        </h1>

        {/* Error Message */}
        <div className="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg p-4 mb-6">
          <p className="text-red-800 dark:text-red-300 text-center font-medium">
            {error.message}
          </p>
        </div>

        {/* Action Buttons */}
        <div className="flex flex-col sm:flex-row gap-3 justify-center mb-6">
          <button
            onClick={() => window.location.reload()}
            className="px-6 py-3 bg-blue-600 hover:bg-blue-700 text-white rounded-lg font-medium transition-colors"
          >
            Refresh Page
          </button>
          <button
            onClick={() => window.location.href = '/'}
            className="px-6 py-3 bg-slate-200 hover:bg-slate-300 dark:bg-slate-700 dark:hover:bg-slate-600 text-slate-900 dark:text-slate-100 rounded-lg font-medium transition-colors"
          >
            Go Home
          </button>
        </div>
        <div className="border-t border-slate-200 dark:border-slate-700 pt-6">
          <button
            onClick={() => setShowDetails(!showDetails)}
            className="w-full flex items-center justify-center gap-2 text-sm text-slate-600 dark:text-slate-400 hover:text-slate-900 dark:hover:text-slate-200 transition-colors"
          >
            <span>{showDetails ? 'Hide' : 'Show'} Technical Details</span>
            <svg
              className={`w-4 h-4 transition-transform ${showDetails ? 'rotate-180' : ''}`}
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
          </button>
          {showDetails && (
            <div className="mt-4 bg-slate-900 dark:bg-slate-950 rounded-lg p-4 overflow-x-auto">
              <pre className="text-xs text-slate-300 font-mono whitespace-pre-wrap break-words">
                {error.stack}
              </pre>
            </div>
          )}
        </div>
      </div>
    </div>
  )
}

export { Error }
