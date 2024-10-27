import * as React from 'react'

export function useSelection() {
  const [start, setStart] = React.useState<number | undefined>(undefined)
  const [end, setEnd] = React.useState<number | undefined>(undefined)

  const clearSelection = React.useCallback(() => {
    setStart(undefined)
    setEnd(undefined)
  }, [])

  const toggle = React.useCallback(
    (index: number) => {
      if (start === undefined && end === undefined) {
        setStart(index)
        setEnd(undefined)
      } else if (start !== undefined && end !== undefined) {
        setStart(index)
        setEnd(undefined)
      } else {
        const [min, max] = (
          [start, end, index].filter(Boolean) as number[]
        ).sort((a, b) => a - b)

        if (min === max) {
          clearSelection()
          return
        }

        setStart(min)
        setEnd(max)
      }
    },
    [start, end],
  )

  return { start, end, clearSelection, toggle }
}
