use futures_util::StreamExt;
use reqwest::{header::HeaderMap, Method};
use reqwest_eventsource::{Event, EventSource};
use serde::{Deserialize, Serialize};

use crate::error::PolestarError;

use super::req::req_stream;

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct CreateChatCompletionStreamResponse {
  pub id: Option<String>,
  pub object: String,
  pub created: u32,
  pub model: String,
  pub choices: Vec<ChatChoiceDelta>,
  pub usage: Option<Usage>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Usage {
  pub prompt_tokens: u32,
  pub completion_tokens: u32,
  pub total_tokens: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChatChoiceDelta {
  pub index: u32,
  pub delta: ChatCompletionResponseStreamMessage,
  pub finish_reason: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChatCompletionResponseStreamMessage {
  pub content: Option<String>,
  pub role: Option<Role>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "lowercase")]
pub enum Role {
  System,
  #[default]
  User,
  Assistant,
  Function,
}

pub async fn stream_string(content: &str) -> String {
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

  let body = body.replace("123", content);

  let mut stream = req_stream(url, Method::POST, headers, Some(body.to_owned()))
    .await
    .unwrap();

  let mut ret_msg = String::new();
  loop {
    let delta = stream_event_source_handler(&mut stream).await;
    match delta {
      Ok(delta) => {
        if let Some(delta) = delta {
          ret_msg.push_str(&delta);
        } else {
          break;
        }
      }
      Err(_e) => {
        break;
      }
    }
  }

  ret_msg
}

pub async fn stream_event_source_handler(
  stream: &mut EventSource,
) -> Result<Option<String>, PolestarError> {
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
