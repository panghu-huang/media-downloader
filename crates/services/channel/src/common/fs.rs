use std::env;
use std::path::PathBuf;
use tokio::fs;

pub async fn temporary_directory() -> anyhow::Result<PathBuf> {
  let system_temp_dir = env::temp_dir();

  let temp_dir = system_temp_dir.join("media-downloader");

  fs::create_dir_all(&temp_dir)
    .await
    .map_err(|e| anyhow::anyhow!("Failed to create temporary directory: {}", e))?;

  Ok(temp_dir)
}
