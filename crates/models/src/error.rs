use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
  #[error("Error querying the database: {0}")]
  QueryError(#[from] sqlx::Error),

  #[error("Error creating the database pool: {0}")]
  PoolCreationError(String),

  #[error("Error getting a connection from the pool: {0}")]
  ConnectionError(String),

  #[error("Error interacting with the database: {0}")]
  InteractionError(String),
}
