mod api;

use self::api::{
  parse_root_type, Detail, ListRequest, Response as UnifiedAPIResponse, TypeItem, UnifiedAPI,
};
use crate::services::DownloadMediaOptions;
use crate::services::MediaChannelExt;
use configuration::UnifiedItemConfig;
use parking_lot::Mutex;
use protocol::channel::MediaKind;
use protocol::channel::MediaMetadata;
use protocol::channel::{MediaPlaylist, MediaPlaylistItem};
use protocol::channel::{SearchMediaRequest, SearchMediaResponse};
use protocol::DownloadProgressReceiver;

pub struct UnifiedMediaService {
  api: UnifiedAPI,
  channel_name: String,
  types: Mutex<Vec<TypeItem>>,
}

#[async_trait::async_trait]
impl MediaChannelExt for UnifiedMediaService {
  fn channel_name(&self) -> &'static str {
    Box::leak(Box::new(self.channel_name.clone()))
  }

  async fn download_media(
    &self,
    options: DownloadMediaOptions,
  ) -> anyhow::Result<DownloadProgressReceiver> {
    let detail = self.get_media_detail(&options.media_id).await?;

    let mut play_url_list = detail.play_url.split('#');

    let number: usize = options.number.unwrap_or(1).try_into()?;

    let url_of_number = play_url_list.nth(number - 1).ok_or_else(|| {
      anyhow::anyhow!(
        "Invalid number {:?} of media {}",
        options.number,
        options.media_id
      )
    })?;

    let m3u8_url = url_of_number.split('$').nth(1).ok_or_else(|| {
      anyhow::anyhow!(
        "Invalid url format found: {} ({}#{:?})",
        url_of_number,
        options.media_id,
        options.number,
      )
    })?;

    let download_opts = crate::common::DownloadMediaOptions {
      download_url: m3u8_url,
      destination_path: &options.destination_path,
    };

    let progress = crate::common::download_media_using_ffmpeg(download_opts).await?;

    Ok(progress)
  }

  async fn get_media_metadata(&self, media_id: &str) -> anyhow::Result<MediaMetadata> {
    let detail = self.get_media_detail(media_id).await?;

    self.ensure_types().await.ok();

    let release_year = detail.year.parse().unwrap_or(0);

    let kind = self.parse_media_kind(detail.type_id);

    Ok(MediaMetadata {
      kind,
      channel: self.channel_name.clone(),
      id: detail.id.to_string(),
      name: detail.name,
      release_year,
      poster_url: detail.picture,
      description: detail.description,
    })
  }

  async fn search_media(
    &self,
    request: &SearchMediaRequest,
  ) -> anyhow::Result<SearchMediaResponse> {
    let request = ListRequest {
      keyword: Some(request.keyword.clone()),
      page: request.page,
      type_id: None,
    };

    log::info!("Search media with: {:#?}", request);

    let search_result = self.api.list(&request).await?;

    self.ensure_types().await.ok();

    let ids = search_result
      .list
      .iter()
      .map(|item| item.id.to_string())
      .collect::<Vec<_>>();

    let items_with_detail = self.get_media_detail_by_ids(ids.as_slice()).await?;

    let items = items_with_detail
      .list
      .iter()
      .map(|detail| {
        let release_year = detail.year.parse().unwrap_or(0);
        let kind = self.parse_media_kind(detail.type_id);

        MediaMetadata {
          kind,
          channel: self.channel_name.clone(),
          id: detail.id.to_string(),
          name: detail.name.clone(),
          release_year,
          poster_url: detail.picture.clone(),
          description: detail.description.clone(),
        }
      })
      .collect();

    let page_size: u32 = search_result.limit.into();

    Ok(SearchMediaResponse {
      items,
      page_size,
      page: search_result.page.into(),
      total: search_result.total,
    })
  }

  async fn get_media_playlist(&self, media_id: &str) -> anyhow::Result<crate::MediaPlaylist> {
    let detail = self.get_media_detail(media_id).await?;

    let playlist: Vec<MediaPlaylistItem> = detail
      .play_url
      .split('#')
      .enumerate()
      .map(|(index, url)| {
        let number = index + 1;
        let name_and_url = url.split('$').collect::<Vec<_>>();

        MediaPlaylistItem {
          number: number as u32,
          text: name_and_url.first().unwrap_or(&"").to_string(),
          url: name_and_url.get(1).unwrap_or(&"").to_string(),
        }
      })
      .collect();

    Ok(MediaPlaylist {
      channel: self.channel_name.clone(),
      media_id: media_id.to_string(),
      items: playlist,
    })
  }
}

impl UnifiedMediaService {
  async fn get_media_detail(&self, id: &str) -> anyhow::Result<Detail> {
    log::info!("Getting video detail of {:?}", id);

    let details = self.get_media_detail_by_ids(&[id]).await?;

    let detail = details.list.first().unwrap();

    Ok(detail.clone())
  }

  async fn get_media_detail_by_ids<T: AsRef<str>>(
    &self,
    ids: &[T],
  ) -> anyhow::Result<UnifiedAPIResponse<Detail>> {
    let details = self.api.get_details(ids).await?;

    if details.list.is_empty() {
      let ids = ids.iter().map(|s| s.as_ref()).collect::<Vec<_>>().join(",");
      anyhow::bail!("Invalid media IDs: {:?}", ids);
    }

    Ok(details)
  }

  fn parse_media_kind(&self, type_id: u32) -> MediaKind {
    let types = self.types.lock();

    parse_root_type(&types, type_id)
  }

  async fn ensure_types(&self) -> anyhow::Result<()> {
    let is_empty = self.types.lock().is_empty();

    if is_empty {
      let res = self
        .api
        .list(&ListRequest {
          page: 1,
          keyword: None,
          type_id: None,
        })
        .await?;

      *self.types.lock() = res.class.unwrap();
    }

    Ok(())
  }
}

impl UnifiedMediaService {
  pub fn new(channel_name: &str, config: &UnifiedItemConfig) -> Self {
    let http_version = config.http_version.unwrap_or(2);

    let api = if http_version == 1 {
      UnifiedAPI::new_with_http_1(&config.base_url)
    } else {
      UnifiedAPI::new(&config.base_url)
    };

    Self {
      channel_name: channel_name.to_owned(),
      api,
      types: Mutex::new(vec![]),
    }
  }
}
