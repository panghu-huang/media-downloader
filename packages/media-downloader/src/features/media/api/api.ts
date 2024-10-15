import { APIClient } from '@/common/api-client'
import { ListResponse } from '@/common/types'
import { MediaMetadata, MediaPlaylistItem } from '@/features/media/types'

export interface SearchMediaOptions {
  keyword: string
  page?: number
}

export type SearchMediaResponse = ListResponse<MediaMetadata>

export type MediaPlaylistResponse = ListResponse<MediaPlaylistItem>

class MediaAPI extends APIClient {
  public async search(options: SearchMediaOptions) {
    const res = await this.request<SearchMediaResponse>({
      url: '/media/search',
      params: options,
    })

    return res
  }

  public async getMetadata(channel: string, id: string) {
    const res = await this.request<MediaMetadata>({
      url: `/channels/${channel}/media/${id}`,
    })

    return res
  }

  public async getPlaylist(channel: string, id: string) {
    const res = await this.request<MediaPlaylistResponse>({
      url: `/channels/${channel}/media/${id}/playlist`,
    })

    return res
  }
}

export { MediaAPI }
