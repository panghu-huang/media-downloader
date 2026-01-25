use m3u8_rs::{parse_playlist_res, MasterPlaylist, MediaPlaylist, Playlist};
use protocol::{DownloadProgressExt, DownloadProgressReceiver, DownloadProgressStream};
use reqwest::Client;
use std::path::Path;
use std::process::Stdio;
use tokio::fs;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use url::{Origin, Url};

pub struct DownloadMediaOptions<'a> {
  pub download_url: &'a str,
  pub destination_path: &'a Path,
}

pub async fn download_media_using_ffmpeg(
  options: DownloadMediaOptions<'_>,
) -> anyhow::Result<DownloadProgressReceiver> {
  let playlist = fetch_media_playlist(options.download_url).await?;

  let total_segments = playlist.segments.len();
  let stream = stream::Stream::new(Ok);

  stream.start(total_segments);
  let receiver = stream.recv();

  tokio::spawn({
    let download_url = options.download_url.to_string();
    let destination_path = options.destination_path.to_path_buf();

    async move {
      log::info!(
        "Downloading media using ffmpeg: {} (total segments: {})",
        download_url,
        total_segments
      );

      if let Err(err) =
        download_with_ffmpeg_progress(&download_url, &destination_path, &stream, total_segments)
          .await
      {
        log::error!("Failed to download with ffmpeg: {}", err);
        stream.failed(&err.to_string());
        return Err(err);
      }

      log::info!("Done. {:?}", destination_path);
      stream.done(&destination_path.to_string_lossy());

      Ok(()) as anyhow::Result<()>
    }
  });

  Ok(receiver)
}

async fn download_with_ffmpeg_progress(
  download_url: &str,
  destination_path: &Path,
  stream: &DownloadProgressStream,
  total_segments: usize,
) -> anyhow::Result<()> {
  fs::create_dir_all(destination_path.parent().unwrap()).await?;

  let mut child = Command::new("ffmpeg")
    .arg("-i")
    .arg(download_url)
    .arg("-c")
    .arg("copy")
    .arg("-y")
    .arg(destination_path)
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()?;

  let stdout = child.stdout.take().unwrap();
  let stderr = child.stderr.take().unwrap();

  // Capture stdout (usually empty for this use case)
  let stdout_handle = tokio::spawn(async move {
    let reader = BufReader::new(stdout);
    let mut lines = reader.lines();
    while let Ok(Some(_line)) = lines.next_line().await {
      // stdout is usually empty when not using -progress
    }
  });

  let stream_clone = stream.clone();
  let stderr_handle = tokio::spawn(async move {
    let reader = BufReader::new(stderr);
    let mut lines = reader.lines();
    let mut last_reported_time = 0.0;
    let mut segment_count = 0;

    while let Ok(Some(line)) = lines.next_line().await {
      log::info!("ffmpeg: {}", line);

      // Count HLS segments being opened
      // Example: "[hls @ 0xaaaabc34aab0] Opening 'crypto+https://...' for reading"
      if line.contains("[hls @") && line.contains("Opening") {
        segment_count += 1;
      }

      // Parse progress from frame output
      // Example: "frame=25200 fps= 56 q=-1.0 size=  262912kB time=00:16:48.04 bitrate=2136.6kbits/s speed=2.23x"
      if line.contains("frame=") && line.contains("time=") {
        if let Some(current_time) = parse_time_from_frame_line(&line) {
          // Only report if time changed significantly (more than 3 seconds)
          if current_time - last_reported_time >= 3.0 {
            let progress_pct = if total_segments > 0 {
              (segment_count as f64 / total_segments as f64 * 100.0).min(99.0) as usize
            } else {
              0
            };

            stream_clone.segment_downloaded(&format!(
              "Downloading: {:.0}m {:.0}s - {}/{} segments ({}%)",
              current_time / 60.0,
              current_time % 60.0,
              segment_count,
              total_segments,
              progress_pct
            ));
            last_reported_time = current_time;
          }
        }
      }

      // Detect completion - look for final summary line
      // Example: "frame=30423 fps= 56 q=-1.0 Lsize=  313424kB time=00:20:16.93 bitrate=2109.9kbits/s speed=2.25x"
      if line.contains("Lsize=") && line.contains("time=") {
        if let Some(final_time) = parse_time_from_frame_line(&line) {
          stream_clone.segment_downloaded(&format!(
            "Completed: {:.0}m {:.0}s - {}/{} segments",
            final_time / 60.0,
            final_time % 60.0,
            segment_count,
            total_segments
          ));
        }
      }
    }
  });

  let status = child.wait().await?;

  stdout_handle.await?;
  stderr_handle.await?;

  if !status.success() {
    anyhow::bail!("ffmpeg command failed with status: {:?}", status);
  }

  Ok(())
}

fn parse_time_from_frame_line(line: &str) -> Option<f64> {
  // Parse line like "frame=25200 fps= 56 q=-1.0 size=  262912kB time=00:16:48.04 bitrate=2136.6kbits/s speed=2.23x"
  if let Some(time_part) = line.split("time=").nth(1) {
    if let Some(time_str) = time_part.split_whitespace().next() {
      return parse_time_string(time_str);
    }
  }
  None
}

fn parse_time_string(time_str: &str) -> Option<f64> {
  // Parse time string like "00:16:48.04"
  let parts: Vec<&str> = time_str.split(':').collect();
  if parts.len() == 3 {
    if let (Ok(hours), Ok(minutes), Ok(seconds)) = (
      parts[0].parse::<f64>(),
      parts[1].parse::<f64>(),
      parts[2].parse::<f64>(),
    ) {
      return Some(hours * 3600.0 + minutes * 60.0 + seconds);
    }
  }
  None
}

#[async_recursion::async_recursion]
async fn fetch_media_playlist(download_url: &str) -> anyhow::Result<MediaPlaylist> {
  log::info!("Starting fetch media playlist: {}", download_url);
  let client = Client::new();

  let res = client.get(download_url).send().await?;

  let status = res.status();
  if !status.is_success() {
    log::info!("{:#?}", res.headers());
    anyhow::bail!("Request failed with code {}", status);
  }

  let bytes = res.bytes().await?.to_vec();
  let parsed = parse_playlist_res(&bytes);
  let origin = extract_origin_from_url(download_url)?;

  let playlist = match parsed {
    Ok(Playlist::MediaPlaylist(mut playlist)) => {
      normalize_media_playlist(&mut playlist, &origin);

      playlist
    }
    Ok(Playlist::MasterPlaylist(mut master_playlist)) => {
      log::info!("Got master playlist: {:#?}", master_playlist);
      normalize_master_playlist(&mut master_playlist, &origin);

      parse_master_playlist(&mut master_playlist).await?
    }
    Err(err) => anyhow::bail!("Fetch media playlist error: {}", err),
  };

  Ok(playlist)
}

async fn parse_master_playlist(
  master_playlist: &mut MasterPlaylist,
) -> anyhow::Result<MediaPlaylist> {
  if let Some(new_variant_stream) = master_playlist.get_newest_variant() {
    fetch_media_playlist(&new_variant_stream.uri).await
  } else {
    anyhow::bail!("Unsupported format")
  }
}

fn normalize_media_playlist(media_playlist: &mut MediaPlaylist, origin: &str) {
  for segment in &mut media_playlist.segments {
    segment.uri = normalize_url(&segment.uri, origin);
  }
}

fn normalize_master_playlist(master_playlist: &mut MasterPlaylist, origin: &str) {
  for variant_stream in &mut master_playlist.variants {
    variant_stream.uri = normalize_url(&variant_stream.uri, origin);
  }
}

fn normalize_url(path_or_url: &str, origin: &str) -> String {
  if path_or_url.starts_with("http") {
    path_or_url.to_string()
  } else {
    format!("{}{}", origin, path_or_url)
  }
}

fn extract_origin_from_url(url: &str) -> anyhow::Result<String> {
  let parsed_url = Url::parse(url)?;
  let Origin::Tuple(protocol, host, _port) = parsed_url.origin() else {
    anyhow::bail!("Unsupported origin");
  };

  Ok(format!("{}://{}", protocol, host))
}
