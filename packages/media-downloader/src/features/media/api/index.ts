import { getRuntimeConfig } from '@coodev/react/config'
import { RuntimeConfig } from '@/common/types'
import { MediaAPI } from './api'

function createMediaAPI() {
  const runtimeConfig = getRuntimeConfig() as RuntimeConfig

  return new MediaAPI(runtimeConfig.apiBaseURL)
}

export const mediaAPI = createMediaAPI()

export type * from './api'
