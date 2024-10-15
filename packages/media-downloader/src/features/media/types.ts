export interface MediaMetadata {
  channel: string
  id: string
  name: string
  poster_url: string
  release_year: number
  description: string
  kind: string
}

export interface MediaPlaylistItem {
  number: number
  text: string
  url: string
}
