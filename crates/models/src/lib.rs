mod error;
mod models;

use std::ops::Deref;

pub use error::DatabaseError;
pub use models::*;

pub type DatabaseResult<T> = Result<T, DatabaseError>;

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

#[derive(Clone)]
pub struct ConnectionPool(pub(crate) PgPool);

impl ConnectionPool {
  pub async fn connect(database_url: &str) -> DatabaseResult<Self> {
    let pool = PgPoolOptions::new()
      .max_connections(5)
      .connect(database_url)
      .await?;

    Ok(Self(pool))
  }
}

impl Deref for ConnectionPool {
  type Target = PgPool;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
