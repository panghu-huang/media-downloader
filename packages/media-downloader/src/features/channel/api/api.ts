import { APIClient } from '@/common/api-client'
import { Channel } from '../types'

export interface GetChannelsResponse {
  channels: Channel[]
}

export class ChannelAPI extends APIClient {
  public async getChannels() {
    const res = await this.request<GetChannelsResponse>({
      url: '/channels',
    })

    return res
  }
}

