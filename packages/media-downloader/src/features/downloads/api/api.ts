import { APIClient } from '@/common/api-client'
import { DownloadTask } from '../types'

class DownloadsAPI extends APIClient {
  public async listDownloads() {
    const res = await this.request<DownloadTask[]>({
      url: '/downloads',
    })

    return res
  }
}

export { DownloadsAPI }
