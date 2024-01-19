use futures_util::{Stream, StreamExt};
use reqwest_eventsource::Event;
use serde::{Deserialize, Serialize};

use crate::{error::PolestarError, model::MsgRole};

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

impl From<MsgRole> for Role {
  fn from(role: MsgRole) -> Self {
    match role {
      MsgRole::System(_) => Role::System,
      MsgRole::User => Role::User,
      MsgRole::Bot(_) => Role::Assistant,
    }
  }
}

pub async fn deal_open_ai_stream(
  stream: &mut (impl Stream<Item = Result<Event, reqwest_eventsource::Error>> + Unpin),
  mut delta_op: impl FnMut(String),
) -> Result<String, PolestarError> {
  let mut answer = String::default();
  loop {
    let delta = stream_event_source_handler(stream).await?;
    if let Some(delta) = delta {
      answer.push_str(delta.as_ref());
      delta_op(delta);
    } else {
      break;
    }
  }
  Ok(answer)
}

pub fn mock_stream_string(_content: &str, mut delta_op: impl FnMut(String)) {
  use rand::{distributions::Alphanumeric, Rng};

  let mut count = 0;
  let max = 10;
  loop {
    if count >= max {
      break;
    }
    let s: String = rand::thread_rng()
      .sample_iter(&Alphanumeric)
      .take(7)
      .map(char::from)
      .collect();
    count += 1;
    delta_op(s);
  }
}

async fn stream_event_source_handler(
  stream: &mut (impl Stream<Item = Result<Event, reqwest_eventsource::Error>> + Unpin),
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
      Err(reqwest_eventsource::Error::StreamEnded) => match delta.is_empty() {
        true => return Ok(None),
        false => return Ok(Some(delta)),
      },
      Err(e) => {
        return Err(PolestarError::EventSource(e));
      }
    }
  }
  Ok(Some(delta))
}
