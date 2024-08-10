use m3u8_rs::{parse_playlist_res, Playlist};
use regex::Regex;
use reqwest::Client;
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::vec;
use std::{path::PathBuf, str::FromStr};
use tokio::fs;
use tokio::sync::Semaphore;
use tracing_subscriber::fmt::time::ChronoLocal;
use url::Url;

const WEB_PAGE_HOST: &str = "xiaoxintv.com";
const MEDIA_DIR: &str = "/home/coodev/projects/media-downloader-b/tv_shows";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let tv_show_id = 548;
  let tv_show_season_id = 1;
  let tv_show_start_nid = 339;
  let tv_show_name = "海贼王";
  let tv_show_year = 1999;

  create_download_dir().await?;

  for idx in 0..61 {
    let tv_show_nid = tv_show_start_nid + idx;

    log::info!("Downloading TV Show {} of {}", tv_show_nid, tv_show_name);

    download_tv_show(
      tv_show_id,
      tv_show_season_id,
      tv_show_nid,
      tv_show_name,
      tv_show_year,
    )
    .await?;
  }

  log::info!("Cleanup download directory.");

  cleanup_download_dir().await?;

  log::info!("Done.");

  Ok(())
}

fn get_tv_show_page_url(tv_show_id: u32, session_id: u32, nid: u32) -> String {
  format!(
    "https://{}/index.php/vod/play/id/{}/sid/{}/nid/{}.html",
    WEB_PAGE_HOST, tv_show_id, session_id, nid
  )
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
    log::info!("{:#?}", res.headers());
    anyhow::bail!("Request failed with code {}", status);
  }

  let bytes = res.bytes().await?.to_vec();

  let download_dir = download_dir();

  let parsed_url = Url::from_str(download_url).unwrap();

  let path_segments = parsed_url
    .path_segments()
    .ok_or_else(|| anyhow::anyhow!("Invalid url {}", download_url))?;

  let file_name = path_segments
    .last()
    .ok_or_else(|| anyhow::anyhow!("Invalid url {}", download_url))?;

  let download_file_name = download_dir.join(file_name);

  fs::write(&download_file_name, bytes).await?;

  Ok(download_file_name)
}

async fn extract_download_url_from_tv_show_page(tv_show_page_url: &str) -> anyhow::Result<String> {
  let client = Client::builder()
    .http2_adaptive_window(true)
    .http2_prior_knowledge()
    .use_rustls_tls()
    .build()?;

  let html = client.get(tv_show_page_url)
    .header(
      "user-agent", 
      "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/127.0.0.0 Safari/537.36"
    )
    .version(http::Version::HTTP_2)
    .send()
    .await?
    .text()
    .await?;

  let reg = Regex::new(r#"","url":"((\S)+)","url_next"#)?;

  let caps = reg.captures(&html);

  let video_download_url = caps.unwrap().get(1).unwrap().as_str().replace("\\/", "/");

  Ok(video_download_url)
}

async fn download_tv_show(
  tv_show_id: u32,
  tv_show_season_id: u32,
  tv_show_nid: u32,
  tv_show_name: &str,
  tv_show_year: u32,
) -> anyhow::Result<()> {
  let tv_show_page_url = get_tv_show_page_url(tv_show_id, tv_show_season_id, tv_show_nid);

  let download_url = extract_download_url_from_tv_show_page(&tv_show_page_url).await?;

  let client = Client::new();

  let bytes = client.get(download_url).send().await?.bytes().await?;

  let bytes: Vec<u8> = bytes.to_vec();

  let parsed = parse_playlist_res(&bytes);

  let Ok(Playlist::MediaPlaylist(mut playlist)) = parsed else {
    anyhow::bail!("Not supported format.");
  };

  let semaphore = Arc::new(Semaphore::new(10));

  let segments = playlist.segments.clone();

  let mut handles = vec![];

  for segment in segments {
    let semaphore = semaphore.clone();

    let handle = tokio::spawn(async move {
      let _permit = semaphore.acquire().await?;

      let downloaded_path = download_media_segment(&segment.uri).await?;

      // Wait 2s for avoid 429 error
      tokio::time::sleep(std::time::Duration::from_secs(2)).await;

      Ok(downloaded_path) as anyhow::Result<PathBuf>
    });

    handles.push(handle);
  }

  let total = handles.len();

  for idx in 0..total {
    let handle = handles.get_mut(idx).unwrap();

    let downloaded_path = handle.await??;

    playlist.segments.get_mut(idx).unwrap().uri = downloaded_path.to_string_lossy().to_string();
  }

  log::info!("Writing to file system ... ");

  let mut bytes: Vec<u8> = Vec::new();

  playlist.write_to(&mut bytes)?;

  let manifest_file_name = format!("{}-{}-{}.m3u8", tv_show_id, tv_show_season_id, tv_show_nid);
  let pl_path = download_dir().join(manifest_file_name);

  fs::write(&pl_path, bytes).await?;

  let dest_path =
    tv_show_destination_path(tv_show_name, tv_show_year, tv_show_season_id, tv_show_nid);

  log::info!("Transforming to mp4 ... ");

  fs::create_dir_all(&dest_path.parent().unwrap()).await?;

  transform_to_mp4(&pl_path, &dest_path).await?;

  log::info!(
    "TV Show {} of {} has been downloaded.",
    tv_show_nid,
    tv_show_name
  );

  Ok(())
}

async fn transform_to_mp4(source: &Path, dest: &Path) -> anyhow::Result<()> {
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
    anyhow::bail!("Invalid exit code");
  }

  Ok(())
}

fn tv_show_destination_path(
  tv_show_name: &str,
  tv_show_year: u32,
  tv_show_season_id: u32,
  tv_show_nid: u32,
) -> PathBuf {
  let media_dir = PathBuf::from(MEDIA_DIR);

  media_dir
    .join(format!("{}.{}", tv_show_name, tv_show_year))
    .join(format!("Season.{}", tv_show_season_id))
    .join(format!(
      "{}.{}.S{}E{}.mp4",
      tv_show_name, tv_show_year, tv_show_season_id, tv_show_nid
    ))
}

async fn create_download_dir() -> anyhow::Result<()> {
  let dir = download_dir();

  fs::create_dir_all(dir).await?;

  Ok(())
}

async fn cleanup_download_dir() -> anyhow::Result<()> {
  let dir = download_dir();

  fs::remove_dir_all(dir).await?;

  Ok(())
}

fn download_dir() -> PathBuf {
  let current_dir = std::env::current_dir().unwrap();

  current_dir.join("downloads")
}
