use polestar_core::{
  error::PolestarError,
  model::FeedbackMessageListForServer,
  service::{
    open_ai::{deal_open_ai_stream, open_ai_stream},
    req::{fetch_feedback, req_feedback},
  },
};
use reqwest::header::HeaderMap;
use ribir::prelude::*;

pub async fn query_open_ai(
  url: String,
  content: String,
  headers: HeaderMap,
  delta_op: impl FnMut(String),
) -> Result<String, PolestarError> {
  let mut stream = open_ai_stream(url, content, headers)
    .to_ribir_future()
    .await
    .unwrap()
    .to_ribir_stream();

  deal_open_ai_stream(&mut stream, delta_op).await
}

pub async fn query_feedback(content: String) {
  let _ = req_feedback(content).to_ribir_future().await;
}

pub async fn query_fetch_feedback(
  utc_time: Option<i64>,
) -> Result<FeedbackMessageListForServer, PolestarError> {
  fetch_feedback(utc_time).to_ribir_future().await
}
