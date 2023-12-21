use polestar_core::service::open_ai::{deal_open_ai_stream, open_ai_stream};
use reqwest::header::HeaderMap;
use ribir::prelude::*;

pub async fn query_open_ai(
  url: String,
  content: String,
  header: HeaderMap,
  delta_op: impl FnMut(String),
) {
  let mut stream = open_ai_stream(url, content, header)
    .to_ribir_future()
    .await
    .unwrap()
    .to_ribir_stream();

  let _ = deal_open_ai_stream(&mut stream, delta_op).await;
}
