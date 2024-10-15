import { type ReactRenderContext } from '@coodev/react/types'

export interface LoaderContext<P = object>
  extends Pick<ReactRenderContext, 'url' | 'req' | 'res'> {
  params: P
}

export * from './response'
export * from './runtime-config'
