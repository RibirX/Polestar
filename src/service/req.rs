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

#[cfg(test)]
mod test {
  use super::*;

  // TODO: remove it.
  #[tokio::test]
  async fn test_open_ai_stream() {
    let version = env!("CARGO_PKG_VERSION");
    let url = "https://api.ribir.org/stream/open_ai/v1/chat/completions";
    let mut headers = HeaderMap::new();
    headers.insert(
      "User-Agent",
      format!("PoleStarChat/{}", version).parse().unwrap(),
    );
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("Authorization", "v1.eyJ1c2VyX2lkIjoxMDAxMDIsImV4cCI6MTY5ODEzMzYxMCwidmVyIjoidjEifQ.CwB5-cvArO_UJVIPSZgb1GMKJ-tFpXOqhJNLg-rPxTY".parse().unwrap());
    headers.insert("Version", version.parse().unwrap());
    let body = r#"{"model":"gpt-3.5-turbo","messages":[{"role":"system","content":"I want you to act as a Chinese translator, spelling corrector and improver. I will speak to you in any language and you will detect the language, translate it and answer in the corrected and improved version of my text, in Chinese. I want you to only reply to corrections, improvements and nothing else, do not write explanation. \nText: ###### "},{"role":"user","content":"123"}],"stream":true}"#;
    let mut stream = req_stream(url, Method::POST, headers, Some(body.to_owned()))
      .await
      .unwrap();

    loop {
      let delta = delta(&mut stream).await;
      match delta {
        Ok(delta) => {
          if let Some(delta) = delta {
            println!("delta: {:?}", delta);
          } else {
            println!("delta: None");
            break;
          }
        }
        Err(e) => {
          println!("error: {:?}", e);
          break;
        }
      }
    }
  }
}
