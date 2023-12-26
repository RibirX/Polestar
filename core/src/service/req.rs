use futures_util::StreamExt;
use regex::Regex;
use reqwest::{header::HeaderMap, Method, RequestBuilder};
use reqwest_eventsource::EventSource;

use crate::{error::PolestarError, model::GLOBAL_VARS};

pub fn req_builder(
  url: &str,
  method: Method,
  headers: HeaderMap,
  body: Option<String>,
) -> RequestBuilder {
  let client = reqwest::Client::new();
  let mut req_builder = client.request(method, url);
  for (key, value) in headers.iter() {
    if let Ok(str) = value.to_str() {
      let value =
        GLOBAL_VARS
          .try_lock()
          .unwrap()
          .iter()
          .fold(String::from(str), |str, (key, value)| {
            let regex = Regex::new(&format!(r"\$\{{{}\}}", key.to_string())).unwrap();
            regex
              .replace_all(&str, format!("${{1}}{}", value))
              .to_string()
          });
      req_builder = req_builder.header(key, value);
      continue;
    }

    req_builder = req_builder.header(key, value);
  }
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
