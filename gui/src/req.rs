use polestar_core::{
  error::{PolestarError, PolestarResult},
  model::{AppInfo, BotId, FeedbackMessageListForServer, Quota},
  service::{
    open_ai::deal_open_ai_stream,
    req::{create_text_request, fetch_feedback, req_feedback, request_quota},
  },
};

use ribir::prelude::*;

pub async fn query_open_ai(
  info: impl StateReader<Value = AppInfo>,
  bot_id: BotId,
  content: String,
  delta_op: impl FnMut(String),
) -> Result<String, PolestarError> {
  let req = { create_text_request(&info.read(), bot_id) };

  let mut stream = req
    .request(content)
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

pub async fn query_quota(token: Option<String>) -> PolestarResult<Quota> {
  request_quota(token).to_ribir_future().await
}
