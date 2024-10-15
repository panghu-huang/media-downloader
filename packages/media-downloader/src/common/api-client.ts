import axios, { AxiosRequestConfig } from 'axios'

class APIClient {
  private readonly baseURL: string

  constructor(baseURL: string) {
    this.baseURL = baseURL
  }

  async request<T>(config: AxiosRequestConfig): Promise<T> {
    const response = await axios({
      ...config,
      baseURL: this.baseURL,
    })

    return response.data
  }
}

export { APIClient }
