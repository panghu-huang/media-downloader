import { getRuntimeConfig } from '@coodev/react/config'
import { RuntimeConfig } from '@/common/types'
import { DownloadsAPI } from './api'

function createDownloadsAPI() {
  const isServer = typeof window === 'undefined'

  const runtimeConfig = getRuntimeConfig() as RuntimeConfig

  const baseUrl = isServer
    ? runtimeConfig.apiBaseURLServer
    : runtimeConfig.apiBaseURL

  return new DownloadsAPI(baseUrl)
}

export const downloadsAPI = createDownloadsAPI()

export type * from './api'
