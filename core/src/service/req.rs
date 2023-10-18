use futures_util::StreamExt;
use reqwest::{header::HeaderMap, Method, RequestBuilder};
use reqwest_eventsource::{Event, EventSource};

use crate::{error::PolestarError, service::open_ai::CreateChatCompletionStreamResponse};

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

pub async fn delta(stream: &mut EventSource) -> Result<Option<String>, PolestarError> {
  let terminated = "[DONE]";
  let chunk_size = 256;
  let items = stream.ready_chunks(chunk_size).next().await;

  let Some(items) = items else { return Ok(None) };

  let mut delta = String::default();
  for item in items {
    match item {
      Ok(event) => {
        if let Event::Message(event) = event {
          if event.data == terminated {
            break;
          }
          let obj =
            serde_json::from_str::<CreateChatCompletionStreamResponse>(&event.data).unwrap();
          let choices = obj.choices;
          assert!(choices.len() == 1);

          if let Some(content) = &choices[0].delta.content {
            delta.push_str(content);
          }
        }
      }
      Err(e) => {
        return Err(PolestarError::EventSource(e));
      }
    }
  }
  Ok(Some(delta))
}
