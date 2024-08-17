use crate::common::temporary_directory;
use m3u8_rs::{parse_playlist_res, MediaPlaylist, Playlist};
use protocol::{DownloadProgressExt, DownloadProgressReceiver, DownloadProgressStream};
use reqwest::Client;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::str::FromStr;
use std::sync::Arc;
use tokio::fs;
use tokio::sync::Semaphore;
use url::Url;

pub struct DownloadMediaOptions<'a> {
  pub download_url: &'a str,
  pub destination_path: &'a Path,
  pub parallel_size: usize,
}

pub async fn download_media(
  options: DownloadMediaOptions<'_>,
) -> anyhow::Result<DownloadProgressReceiver> {
  let playlist = fetch_media_playlist(options.download_url).await?;

  let total_segments = playlist.segments.len();
  let download_progress_stream = stream::Stream::new(Ok);

  download_progress_stream.start(total_segments);

  let receiver = download_progress_stream.recv();

  tokio::spawn({
    let parallel_size = options.parallel_size;
    let destination_path = options.destination_path.to_path_buf();

    async move {
      if let Err(err) = download_and_transform(
        playlist,
        &download_progress_stream,
        parallel_size,
        destination_path,
      )
      .await
      {
        log::error!("Failed download video: {}", err);
        download_progress_stream.done();

        anyhow::bail!(err)
      }

      Ok(()) as anyhow::Result<()>
    }
  });

  Ok(receiver)
}

async fn download_and_transform(
  playlist: MediaPlaylist,
  download_progress_stream: &DownloadProgressStream,
  parallel_size: usize,
  destination_path: PathBuf,
) -> anyhow::Result<()> {
  log::info!("Downloading segments ... ");
  let updated_playlist =
    download_segments_of_playlist(playlist, parallel_size, download_progress_stream)
      .await
      .map_err(|e| anyhow::anyhow!("Download segments failed: {}", e))?;

  log::info!("Writing to file system ... ");

  let mut bytes: Vec<u8> = Vec::new();

  updated_playlist.write_to(&mut bytes)?;

  let destination_file_name = destination_path.file_name().unwrap();

  let m3u8_file_name = format!("{}.m3u8", destination_file_name.to_str().unwrap());
  let m3u8_path = temporary_directory().await?.join(m3u8_file_name);

  fs::write(&m3u8_path, bytes).await?;

  download_progress_stream.in_progress("Transforming video ... ");

  log::info!("Transforming video ... ");

  fs::create_dir_all(destination_path.parent().unwrap()).await?;

  if let Err(err) = transform_video(&m3u8_path, &destination_path).await {
    log::error!("Failed to tranform video: {}", err);
  }

  download_progress_stream.done();

  Ok(()) as anyhow::Result<()>
}

async fn download_segments_of_playlist(
  mut playlist: MediaPlaylist,
  parallel_size: usize,
  stream: &DownloadProgressStream,
) -> anyhow::Result<MediaPlaylist> {
  let semaphore = Arc::new(Semaphore::new(parallel_size));

  let mut handles = vec![];

  for (idx, segment) in playlist.segments.iter().enumerate() {
    let semaphore = semaphore.clone();

    let handle = tokio::spawn({
      let stream = stream.clone();
      let uri = segment.uri.clone();

      async move {
        let _permit = semaphore.acquire().await?;

        let downloaded_path = download_media_segment(&uri).await?;

        stream.in_progress(&format!("Segment downloaded: {}", idx));

        // Wait 2s for avoid 429 error
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;

        Ok(downloaded_path) as anyhow::Result<PathBuf>
      }
    });

    handles.push(handle);
  }

  let total = handles.len();

  for idx in 0..total {
    let handle = handles.get_mut(idx).unwrap();

    let downloaded_path = handle
      .await
      .map_err(|e| anyhow::anyhow!("Error occurred during download segment '{}': {}", idx, e))??;

    playlist.segments.get_mut(idx).unwrap().uri = downloaded_path.to_string_lossy().to_string();
  }

  Ok(playlist)
}

async fn fetch_media_playlist(download_url: &str) -> anyhow::Result<MediaPlaylist> {
  let client = Client::new();

  let res = client.get(download_url).send().await?;

  let status = res.status();
  if !status.is_success() {
    log::info!("{:#?}", res.headers());
    anyhow::bail!("Request failed with code {}", status);
  }

  let bytes = res.bytes().await?.to_vec();
  let parsed = parse_playlist_res(&bytes);

  let playlist = match parsed {
    Ok(Playlist::MediaPlaylist(playlist)) => playlist,
    Ok(Playlist::MasterPlaylist(_)) => anyhow::bail!("Unsupported format"),
    Err(err) => anyhow::bail!("Fetch media playlist error: {}", err),
  };

  Ok(playlist)
}

async fn download_media_segment(download_url: &str) -> anyhow::Result<PathBuf> {
  let client = Client::builder()
    .http2_adaptive_window(true)
    .http2_prior_knowledge()
    .use_rustls_tls()
    .build()?;

  let res = client.get(download_url).send().await?;

  let status = res.status();
  if !status.is_success() {
    log::error!("Request failed with code {}", status);
    log::error!("{:#?}", res.headers());

    anyhow::bail!("Request failed with code {}", status);
  }

  let bytes = res.bytes().await?.to_vec();

  let temp_dir = temporary_directory().await?;

  let parsed_url = Url::from_str(download_url).unwrap();

  let path_segments = parsed_url
    .path_segments()
    .ok_or_else(|| anyhow::anyhow!("Invalid url {}", download_url))?;

  let file_name = path_segments
    .last()
    .ok_or_else(|| anyhow::anyhow!("Invalid url {}", download_url))?;

  let download_file_name = temp_dir.join(file_name);

  fs::write(&download_file_name, bytes).await?;

  Ok(download_file_name)
}

async fn transform_video(source: &Path, dest: &Path) -> anyhow::Result<()> {
  let status = Command::new("ffmpeg")
    .arg("-i")
    .arg(source)
    .arg("-codec")
    .arg("copy")
    .arg(dest)
    .stdout(Stdio::null())
    .stderr(Stdio::null())
    .status()?;

  if !status.success() {
    anyhow::bail!("invalid exit code");
  }

  Ok(())
}
