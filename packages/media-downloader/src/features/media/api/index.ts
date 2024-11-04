import { getRuntimeConfig } from '@coodev/react/config'
import { RuntimeConfig } from '@/common/types'
import { MediaAPI } from './api'

function createMediaAPI() {
  const isServer = typeof window === 'undefined'

  const runtimeConfig = getRuntimeConfig() as RuntimeConfig

  const baseUrl = isServer ? runtimeConfig.apiBaseURLServer : runtimeConfig.apiBaseURL

  return new MediaAPI(baseUrl)
}

export const mediaAPI = createMediaAPI()

export type * from './api'
