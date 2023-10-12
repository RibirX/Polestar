use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
  #[error("sqlite error: {0}")]
  Sqlite(#[from] sqlx::Error),
  #[error("io error: {0}")]
  IO(#[from] std::io::Error),
  #[error("json error: {0}")]
  Json(#[from] serde_json::Error),
}
