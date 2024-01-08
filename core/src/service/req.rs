use futures_util::StreamExt;
use regex::Regex;
use reqwest::{
  header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE, USER_AGENT},
  Method, RequestBuilder,
};
use reqwest_eventsource::EventSource;

use crate::{
  error::PolestarError,
  model::{
    FeedbackMessageListForServer, FeedbackTimestamp, GlbVar, UserFeedbackMessageForServer,
    GLOBAL_VARS,
  },
};

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
