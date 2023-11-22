use thiserror::Error;

#[derive(Error, Debug)]
pub enum PolestarError {
  #[error("io error: {0}")]
  IO(#[from] std::io::Error),
  #[error("json error: {0}")]
  Json(#[from] serde_json::Error),
  #[error("reqwest error: {0}")]
  Reqwest(#[from] reqwest::Error),
  #[error("eventsource error: {0}")]
  EventSource(#[from] reqwest_eventsource::Error),
  #[error("database not found")]
  DatabaseNotFound,
  #[error("database error: {0}")]
  Database(#[from] sqlx::Error),
  #[error("utf8 parser error: {0}")]
  UTF8(#[from] std::string::FromUtf8Error),
}

pub type PolestarResult<T> = Result<T, PolestarError>;
