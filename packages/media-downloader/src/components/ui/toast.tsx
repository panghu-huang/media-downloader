import * as React from 'react'
import { CheckCircle2, XCircle, Info, X } from 'lucide-react'
import { cn } from '@/common/utils'

type ToastType = 'success' | 'error' | 'info'

interface Toast {
  id: string
  message: string
  type: ToastType
}

interface ToastContextType {
  toast: (message: string, type?: ToastType) => void
}

const ToastContext = React.createContext<ToastContextType | undefined>(undefined)

export const useToast = () => {
  const context = React.useContext(ToastContext)
  if (!context) {
    throw new Error('useToast must be used within ToastProvider')
  }
  return context
}

export const ToastProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [toasts, setToasts] = React.useState<Toast[]>([])

  const toast = React.useCallback((message: string, type: ToastType = 'success') => {
    const id = Math.random().toString(36).substring(7)
    setToasts(prev => [...prev, { id, message, type }])

    setTimeout(() => {
      setToasts(prev => prev.filter(t => t.id !== id))
    }, 3000)
  }, [])

  const removeToast = (id: string) => {
    setToasts(prev => prev.filter(t => t.id !== id))
  }

  return (
    <ToastContext.Provider value={{ toast }}>
      {children}
      <div className="fixed bottom-4 right-4 z-50 flex flex-col gap-2 pointer-events-none">
        {toasts.map(t => (
          <ToastItem key={t.id} toast={t} onClose={() => removeToast(t.id)} />
        ))}
      </div>
    </ToastContext.Provider>
  )
}

const ToastItem: React.FC<{ toast: Toast; onClose: () => void }> = ({ toast, onClose }) => {
  const [isVisible, setIsVisible] = React.useState(false)

  React.useEffect(() => {
    requestAnimationFrame(() => {
      setIsVisible(true)
    })
  }, [])

  const icons = {
    success: CheckCircle2,
    error: XCircle,
    info: Info,
  }

  const Icon = icons[toast.type]

  return (
    <div
      className={cn(
        'pointer-events-auto flex items-center gap-3 rounded-lg px-4 py-3 shadow-lg transition-all duration-300 min-w-[300px]',
        'transform',
        isVisible ? 'translate-x-0 opacity-100' : 'translate-x-full opacity-0',
        toast.type === 'success' && 'bg-green-50 dark:bg-green-950 border border-green-200 dark:border-green-800',
        toast.type === 'error' && 'bg-red-50 dark:bg-red-950 border border-red-200 dark:border-red-800',
        toast.type === 'info' && 'bg-blue-50 dark:bg-blue-950 border border-blue-200 dark:border-blue-800'
      )}
    >
      <Icon
        className={cn(
          'h-5 w-5 flex-shrink-0',
          toast.type === 'success' && 'text-green-600 dark:text-green-400',
          toast.type === 'error' && 'text-red-600 dark:text-red-400',
          toast.type === 'info' && 'text-blue-600 dark:text-blue-400'
        )}
      />
      <p
        className={cn(
          'text-sm font-medium flex-1',
          toast.type === 'success' && 'text-green-900 dark:text-green-100',
          toast.type === 'error' && 'text-red-900 dark:text-red-100',
          toast.type === 'info' && 'text-blue-900 dark:text-blue-100'
        )}
      >
        {toast.message}
      </p>
      <button
        onClick={onClose}
        className={cn(
          'rounded-md p-1 hover:bg-black/5 dark:hover:bg-white/5 transition-colors',
          toast.type === 'success' && 'text-green-600 dark:text-green-400',
          toast.type === 'error' && 'text-red-600 dark:text-red-400',
          toast.type === 'info' && 'text-blue-600 dark:text-blue-400'
        )}
      >
        <X className="h-4 w-4" />
      </button>
    </div>
  )
}
