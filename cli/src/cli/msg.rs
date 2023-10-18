use reedline_repl_rs::{clap::ArgMatches, Result as ReplResult};
use reqwest::{header::HeaderMap, Method};

use polestar_chat_core::service::req::{delta, req_stream};

pub async fn msg_handler<T>(args: ArgMatches, _context: &mut T) -> ReplResult<Option<String>> {
  match args.subcommand() {
    Some(("send", args)) => {
      let content = args.get_one::<String>("questions").unwrap();

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

      let body = body.replace("123", &content);

      let mut stream = req_stream(url, Method::POST, headers, Some(body.to_owned()))
        .await
        .unwrap();

      let mut ret_msg = String::new();
      loop {
        let delta = delta(&mut stream).await;
        match delta {
          Ok(delta) => {
            if let Some(delta) = delta {
              ret_msg.push_str(&delta);
            } else {
              break;
            }
          }
          Err(e) => {
            break;
          }
        }
      }

      Ok(Some(ret_msg))
    }
    _ => Ok(None),
  }
}
