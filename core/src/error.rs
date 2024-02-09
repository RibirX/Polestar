use serde::{Deserialize, Serialize};
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
  EventSource(#[from] eventsource_stream::EventStreamError<reqwest::Error>),
  #[error("database not found")]
  DatabaseNotFound,
  #[error("database error: {0}")]
  Database(#[from] sqlx::Error),
  #[error("utf8 parser error: {0}")]
  UTF8(#[from] std::string::FromUtf8Error),
  #[error("Token not found")]
  TokenNotFound,
  #[error("{}: {}.", .0.message, "Please try again later or contact us at Discord")]
  PolestarServerError(PolestarServerError),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PolestarServerError {
  pub kind: PolestarServerErrType,
  pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, PartialOrd)]
pub enum PolestarServerErrType {
  /// [no retry] unauthorized access to the token
  UnAuthed,
  /// [no retry] token expired need to refresh
  Expires,
  /// [no retry] quota been exceeded
  OverQuota,
  /// [no retry] request parameter invalid
  InvalidContent,
  /// [no retry] server internal error
  ServerError,
  /// [retry] response service trigger network error
  NetWork,
  /// [no retry] not found the resource
  NotFound,
  /// [no retry] undefined server error can't retry
  Unknown,
  /// [retry] local request timeout can retry
  TimedOut,
  /// [no retry] internal error
  InternalError,
  /// [no retry] attachment not found
  AttachmentNotFound,
}

impl ToString for PolestarServerErrType {
  fn to_string(&self) -> String {
    match self {
      PolestarServerErrType::UnAuthed => "AppError UnAuthed".to_string(),
      PolestarServerErrType::Expires => "AppError Expires".to_string(),
      PolestarServerErrType::OverQuota => "AppError OverQuota".to_string(),
      PolestarServerErrType::InvalidContent => "AppError InvalidContent".to_string(),
      PolestarServerErrType::ServerError => "AppError ServerError".to_string(),
      PolestarServerErrType::NetWork => "AppError NetWork".to_string(),
      PolestarServerErrType::NotFound => "AppError NotFound".to_string(),
      PolestarServerErrType::Unknown => "AppError Unknown".to_string(),
      PolestarServerErrType::TimedOut => "AppError TimedOut".to_string(),
      PolestarServerErrType::InternalError => "AppError InternalError".to_string(),
      PolestarServerErrType::AttachmentNotFound => "AppError AttachmentNotFound".to_string(),
    }
  }
}

pub type PolestarResult<T> = Result<T, PolestarError>;
