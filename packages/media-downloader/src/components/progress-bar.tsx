import * as React from 'react'

export interface ProgressBarContextType {
  start: () => void
  complete: () => void
}

// 全局进度条控制器
class ProgressBarController {
  private listeners: Set<(action: 'start' | 'complete') => void> = new Set()

  start() {
    this.listeners.forEach(listener => listener('start'))
  }

  complete() {
    this.listeners.forEach(listener => listener('complete'))
  }

  subscribe(listener: (action: 'start' | 'complete') => void) {
    this.listeners.add(listener)
    return () => this.listeners.delete(listener)
  }
}

export const progressBarController = new ProgressBarController()

const ProgressBarContext = React.createContext<ProgressBarContextType | null>(
  null,
)

export const useProgressBar = () => {
  const context = React.useContext(ProgressBarContext)
  if (!context) {
    throw new Error('useProgressBar must be used within ProgressBarProvider')
  }
  return context
}

export const ProgressBarProvider: React.FC<{ children: React.ReactNode }> = ({
  children,
}) => {
  const [progress, setProgress] = React.useState(0)
  const [isVisible, setIsVisible] = React.useState(false)
  const timerRef = React.useRef<NodeJS.Timeout>()
  const hideTimerRef = React.useRef<NodeJS.Timeout>()

  const start = React.useCallback(() => {
    setIsVisible(true)
    setProgress(0)

    if (timerRef.current) {
      clearTimeout(timerRef.current)
    }
    if (hideTimerRef.current) {
      clearTimeout(hideTimerRef.current)
    }

    // 使用 requestAnimationFrame 立即触发重排，然后设置目标值让 CSS 过渡处理
    requestAnimationFrame(() => {
      requestAnimationFrame(() => {
        setProgress(80)
      })
    })
  }, [])

  const complete = React.useCallback(() => {
    if (timerRef.current) {
      clearTimeout(timerRef.current)
    }

    // 立即到100%
    setProgress(100)

    // 500ms后隐藏
    hideTimerRef.current = setTimeout(() => {
      setIsVisible(false)
      setProgress(0)
    }, 500)
  }, [])

  React.useEffect(() => {
    // 订阅全局控制器
    const unsubscribe = progressBarController.subscribe(action => {
      if (action === 'start') {
        start()
      } else if (action === 'complete') {
        complete()
      }
    })

    return () => {
      unsubscribe()
      if (timerRef.current) {
        clearTimeout(timerRef.current)
      }
      if (hideTimerRef.current) {
        clearTimeout(hideTimerRef.current)
      }
    }
  }, [start, complete])

  return (
    <ProgressBarContext.Provider value={{ start, complete }}>
      {isVisible && (
        <div
          className="fixed top-0 left-0 right-0 z-50 bg-blue-600"
          style={{
            height: '3px',
            width: `${progress}%`,
            opacity: progress === 100 ? 0 : 1,
            transition:
              progress === 0
                ? 'none'
                : progress === 100
                  ? 'width 0.3s ease-out, opacity 0.2s ease-out'
                  : 'width 2s ease-out',
          }}
        />
      )}
      {children}
    </ProgressBarContext.Provider>
  )
}
