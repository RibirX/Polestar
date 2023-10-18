use thiserror::Error;

#[derive(Debug, Error)]
pub enum PolestarError {
  #[error("sqlite error: {0}")]
  Sqlite(#[from] sqlx::Error),
  #[error("io error: {0}")]
  IO(#[from] std::io::Error),
  #[error("json error: {0}")]
  Json(#[from] serde_json::Error),
  #[error("reqwest error: {0}")]
  Reqwest(#[from] reqwest::Error),
  #[error("eventsource error: {0}")]
  EventSource(#[from] reqwest_eventsource::Error),
}
