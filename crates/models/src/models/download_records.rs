use crate::models::{DateTime, Id};
use crate::{ConnectionPool, DatabaseResult};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::Type)]
pub enum DownloadStatus {
  Pending,
  InProgress,
  Completed,
  Failed,
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct DownloadRecord {
  pub id: Id,
  pub channel: String,
  pub media_id: String,
  pub status: DownloadStatus,
  pub created_at: DateTime,
  pub updated_at: DateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewDownloadRecord {
  pub channel: String,
  pub media_id: String,
}

impl DownloadRecord {
  pub async fn create(
    pool: &ConnectionPool,
    new_download_record: NewDownloadRecord,
  ) -> DatabaseResult<DownloadRecord> {
    let download_record: DownloadRecord = sqlx::query_as(
      r#"
      INSERT INTO downloads (channel, media_id)
      VALUES ($1, $2)
      RETURNING *
      "#,
    )
    .bind(new_download_record.channel)
    .bind(new_download_record.media_id)
    .fetch_one(&pool.0)
    .await?;

    Ok(download_record)
  }
}
