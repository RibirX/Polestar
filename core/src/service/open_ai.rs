use futures_util::StreamExt;
use reqwest::{header::HeaderMap, Method};
use reqwest_eventsource::{Event, EventSource};
use serde::{Deserialize, Serialize};

use crate::{error::PolestarError, model::Bot};

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

// TODO: add msg context
pub async fn stream_string(content: &str, bot: &Bot, mut delta_op: impl FnMut(String)) {
  // TODO: model need to be configurable
  let body = format!(
    r#"{{"model":"gpt-3.5-turbo","messages":[{{"role":"user","content":"{}"}}],"stream":true}}"#,
    content
  );

  let headers = bot.headers().try_into().unwrap();
  let mut stream = req_stream(bot.url(), Method::POST, headers, Some(body.to_owned()))
    .await
    .unwrap();

  loop {
    let delta = stream_event_source_handler(&mut stream).await;
    if let Ok(Some(delta)) = delta {
      delta_op(delta);
    } else {
      break;
    }
  }
}

async fn stream_event_source_handler(
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
