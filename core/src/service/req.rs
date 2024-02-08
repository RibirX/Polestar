use eventsource_stream::{Event, Eventsource};
use futures_util::{Stream, TryStreamExt};
use log::warn;
use regex::Regex;
use reqwest::{
  header::{HeaderMap, HeaderName, HeaderValue, AUTHORIZATION, CONTENT_TYPE, USER_AGENT},
  Method, RequestBuilder, StatusCode,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, value::Value as JsonValue};
use serde_json_path::JsonPath;

use crate::{
  error::{PolestarError, PolestarResult, PolestarServerError},
  model::{
    AppInfo, Bot, BotId, Channel, FeedbackMessageListForServer, FeedbackTimestamp, GlbVar, Quota,
    ServerProvider, UserFeedbackMessageForServer, GLOBAL_VARS,
  },
};

use super::open_ai::{ChatCompletionResponseStreamMessage, Role};

const POLESTAR_STREAM_URL: &str = "https://api.ribir.org/stream/open_ai";

pub fn req_builder(
  url: &str,
  method: Method,
  headers: HeaderMap,
  body: Option<String>,
) -> RequestBuilder {
  let client = reqwest::Client::new();
  let mut req_builder = client.request(method, url);
  for (key, value) in headers.iter() {
    req_builder = req_builder.header(key, value);
  }
  if let Some(body) = body {
    req_builder = req_builder.body(body);
  }
  req_builder
}

async fn req_stream(
  url: String,
  method: Method,
  headers: HeaderMap,
  body: Option<String>,
) -> Result<impl Stream<Item = Result<Event, PolestarError>>, PolestarError> {
  let req_builder = req_builder(&url, method, headers, body);
  let resp = req_builder.send().await?;
  let content = resp.headers().get("content-type");
  let stream_content = "text/event-stream";
  let content_type = content.and_then(|t| t.to_str().ok());
  if resp.status() == StatusCode::OK && content_type == Some(stream_content) {
    let eventsource = resp.bytes_stream().eventsource();
    Ok(eventsource.map_err(|e| e.into()))
  } else {
    Err(PolestarError::PolestarServerError(
      resp.json::<PolestarServerError>().await?,
    ))
  }
}

pub fn create_text_request(info: &AppInfo, bot_id: BotId) -> TextStreamReq {
  let bot = info.bot(&bot_id).unwrap();
  let sp_name = bot.sp();
  let sp = info.providers().get(sp_name);
  if let Some(sp) = sp {
    create_req_from_bot(bot, Some(sp))
  } else {
    create_req_from_bot(bot, default_polestar_provider(sp_name, info).as_ref())
  }
}

pub fn open_ai_request_content<'a>(bot: &'a Bot, channel: &'a Channel, content: &'a str) -> String {
  let mut messages = vec![];
  if let Some(prompt) = bot.params().get("prompt").and_then(|v| v.as_str()) {
    messages.push(ChatCompletionResponseStreamMessage {
      content: Some(prompt.to_string()),
      role: Some(super::open_ai::Role::System),
    });
  }
  let context_numbers = channel.cfg().mode().context_number();
  let content_with_context = channel
    .msgs()
    .iter()
    .rev()
    .skip(2)
    .take(context_numbers)
    .rev()
    .map(|m| {
      let quote_text = m.meta().quote_id().and_then(|id| {
        channel
          .msg(id)
          .and_then(|m| m.cur_cont_ref().text().map(|s| s.to_owned()))
      });
      let cont_text = m.cur_cont_ref().text().map(|s| s.to_owned());
      ChatCompletionResponseStreamMessage {
        content: Some(quote_text.unwrap_or_default() + &cont_text.unwrap_or_default()),
        role: Some(Role::from(m.role().clone())),
      }
    })
    .collect::<Vec<_>>();
  messages.extend(content_with_context);
  messages.push(ChatCompletionResponseStreamMessage {
    content: Some(content.to_owned()),
    role: Some(Role::User),
  });

  // TODO:
  let params = json!({
    "model": "gpt-3.5-turbo",
    "messages": messages,
    "stream": true,
  });

  serde_json::to_string(&params).unwrap_or_default()
}

fn default_polestar_provider(model: &str, info: &AppInfo) -> Option<ServerProvider> {
  if model == "OpenAI" {
    if let Some(polestar_token) = info.user().and_then(|user| user.token()) {
      return Some(ServerProvider {
        name: "Polestar".to_string(),
        base_url: POLESTAR_STREAM_URL.to_string(),
        token: polestar_token.to_owned(),
        extend: None,
      });
    }
  }

  None
}

fn req_context(sp: Option<&ServerProvider>) -> JsonValue {
  let glb = GLOBAL_VARS.try_lock().unwrap();
  json!({
    "version": glb.get(&GlbVar::Version).unwrap(),
    "user_agent": glb.get(&GlbVar::UserAgent).unwrap(),
    "sp": sp
  })
}

pub async fn fetch_feedback(
  utc_time: Option<i64>,
) -> Result<FeedbackMessageListForServer, PolestarError> {
  let query = if let Some(time_stamp) = utc_time {
    format!(
      "https://api.ribir.org/feedback/messages/?after={}&limit=100",
      time_stamp
    )
  } else {
    "https://api.ribir.org/feedback/messages/?limit=100".to_string()
  };

  let client = reqwest::Client::new();
  let req = {
    let glb = GLOBAL_VARS.lock().unwrap();
    client
      .get(&query)
      .header(AUTHORIZATION, glb.get(&GlbVar::PolestarKey).unwrap())
      .header(CONTENT_TYPE, "application/json")
      .header("Version", glb.get(&GlbVar::Version).unwrap())
      .header(USER_AGENT, glb.get(&GlbVar::UserAgent).unwrap())
  };

  let res = req.send().await;
  match res {
    Ok(res) => {
      let rst = res.json::<FeedbackMessageListForServer>().await;
      match rst {
        Ok(data) => Ok(data),
        Err(err) => Err(PolestarError::Reqwest(err)),
      }
    }
    Err(err) => Err(PolestarError::Reqwest(err)),
  }
}

pub async fn req_feedback(content: String) -> Result<(), PolestarError> {
  let client = reqwest::Client::new();
  let data = UserFeedbackMessageForServer { message: content };
  let params = serde_json::to_string(&data).unwrap();
  let req = {
    let glb = GLOBAL_VARS.lock().unwrap();
    client
      .post("https://api.ribir.org/feedback/ask")
      .header(AUTHORIZATION, glb.get(&GlbVar::PolestarKey).unwrap())
      .header(CONTENT_TYPE, "application/json")
      .header("Version", glb.get(&GlbVar::Version).unwrap())
      .header(USER_AGENT, glb.get(&GlbVar::UserAgent).unwrap())
      .body(params)
  };
  let res = req.send().await;

  match res {
    Ok(res) => {
      let rst = res.json::<FeedbackTimestamp>().await;
      match rst {
        Ok(_) => Ok(()),
        Err(err) => Err(PolestarError::Reqwest(err)),
      }
    }
    Err(err) => Err(PolestarError::Reqwest(err)),
  }
}

#[derive(Debug)]
pub struct TextStreamReq {
  url: String,
  headers: HeaderMap,
}

impl TextStreamReq {
  pub async fn request(
    self,
    body: String,
  ) -> Result<impl Stream<Item = Result<Event, PolestarError>>, PolestarError> {
    req_stream(
      self.url.clone(),
      Method::POST,
      self.headers.clone(),
      Some(body),
    )
    .await
  }
}

fn create_req_from_bot(bot: &Bot, sp: Option<&ServerProvider>) -> TextStreamReq {
  let env = req_context(sp);
  let mut headers: HeaderMap = HeaderMap::default();
  let regex = Regex::new(r"\$\{\s*([^}]*)\s*\}").unwrap();
  for (key, val) in bot.headers().iter() {
    let key = replace_val(key, &regex, &env);
    let val = replace_val(val, &regex, &env);
    headers.insert::<HeaderName>(
      (&key).try_into().unwrap(),
      HeaderValue::from_str(&val).unwrap(),
    );
  }
  let mut url = replace_val(bot.url(), &regex, &env);
  if let Some(base_url) = JsonPath::parse("$.sp.base_url")
    .ok()
    .and_then(|path| path.query(&env).exactly_one().ok())
    .map(to_value_str)
  {
    url = format!("{}{}", base_url, url);
  }
  TextStreamReq { url, headers }
}

fn replace_val(src: &str, path_rex: &Regex, env: &JsonValue) -> String {
  if env.is_null() {
    return src.to_string();
  };
  let caps = path_rex.captures_iter(src);
  let mut pos = 0;
  let mut val = String::default();
  for cap in caps {
    let rg = cap.get(0).unwrap().range();
    val.push_str(&src[pos..rg.start]);
    pos = rg.end;

    if let Some(replaced) = JsonPath::parse(&cap[1])
      .ok()
      .and_then(|path| path.query(env).exactly_one().ok())
    {
      val.push_str(&to_value_str(replaced));
    } else {
      val.push_str(&src[rg.start..rg.end]);
    }
  }
  val.push_str(&src[pos..src.len()]);
  val
}

fn to_value_str(val: &JsonValue) -> String {
  match val {
    JsonValue::String(s) => s.to_string(),
    JsonValue::Number(n) => n.to_string(),
    JsonValue::Bool(b) => b.to_string(),
    _ => {
      warn!("unsupported header value type: {}", val);
      String::default()
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserQuota {
  // pub user_id: u64,
  pub statistics: serde_json::Value,
}

pub async fn request_quota(token: Option<String>) -> PolestarResult<Quota> {
  if let Some(token) = token {
    let client = reqwest::Client::new();
    let res = client
      .get("https://api.ribir.org/user_quota")
      .header(AUTHORIZATION, token)
      .send()
      .await?;
    let user_quota = res.json::<UserQuota>().await?;
    let quota = serde_json::from_value::<Quota>(user_quota.statistics)?;
    Ok(quota)
  } else {
    Err(PolestarError::TokenNotFound)
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn req_replace_val() {
    use super::*;
    let env = req_context(Some(&ServerProvider {
      name: "Polestar".to_string(),
      base_url: "https://api.ribir.org/stream/open_ai".to_string(),
      token: "abc".to_string(),
      extend: Some(json!({ "data": "hello"})),
    }));
    let regex = Regex::new(r"\$\{\s*([^}]*)\s*\}").unwrap();
    let src = r#"${$.sp.name} request to ${$.sp.base_url}/test ${$.sp.extend.data}"#.to_string();
    let replaced = replace_val(&src, &regex, &env);
    assert_eq!(
      replaced,
      "Polestar request to https://api.ribir.org/stream/open_ai/test hello"
    );
  }
}
