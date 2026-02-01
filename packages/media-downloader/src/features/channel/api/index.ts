import { getRuntimeConfig } from '@coodev/react/config'
import { RuntimeConfig } from '@/common/types'
import { ChannelAPI } from './api'

function createChannelAPI() {
  const isServer = typeof window === 'undefined'

  const runtimeConfig = getRuntimeConfig() as RuntimeConfig

  const baseUrl = isServer ? runtimeConfig.apiBaseURLServer : runtimeConfig.apiBaseURL

  return new ChannelAPI(baseUrl)
}

export const channelAPI = createChannelAPI()

export type * from './api'

