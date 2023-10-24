use futures_util::StreamExt;
use reqwest::{header::HeaderMap, Method, RequestBuilder};
use reqwest_eventsource::EventSource;

use crate::error::PolestarError;

pub fn req_builder(
  url: &str,
  method: Method,
  headers: HeaderMap,
  body: Option<String>,
) -> RequestBuilder {
  let client = reqwest::Client::new();
  let mut req_builder = client.request(method, url);
  req_builder = req_builder.headers(headers);
  if let Some(body) = body {
    req_builder = req_builder.body(body);
  }
  req_builder
}

pub async fn req_stream(
  url: &str,
  method: Method,
  headers: HeaderMap,
  body: Option<String>,
) -> Result<EventSource, PolestarError> {
  let req_builder = req_builder(url, method, headers, body);
  let mut stream = EventSource::new(req_builder).unwrap();
  let stream_resp = stream.next().await;
  if let Some(Err(err)) = stream_resp {
    return Err(PolestarError::EventSource(err));
  }
  Ok(stream)
}
