use std::{borrow::Cow, collections::HashMap, fs, io::Read};

use regex::Regex;
use serde::Deserialize;

use super::{launch::write_default_bot_config, project_bot_config_path};
use crate::{model::Bot, service::service_provider::ServiceModel};

#[derive(Deserialize, Debug)]
pub struct BotCfg {
  pub bots: Vec<Bot>,
  pub vars: Option<HashMap<String, String>>,
}

impl BotCfg {
  pub fn has_official_server(&self) -> bool {
    self.bots.iter().any(|b| b.sp() == &ServiceModel::Polestar)
  }
}

pub fn load_bot_cfg_file() -> BotCfg {
  let path = project_bot_config_path();

  fs::File::open(&path)
    .and_then(|mut file| {
      let mut content = String::new();
      file.read_to_string(&mut content).and_then(|_| {
        let bot_cfg = serde_json::from_str::<BotCfg>(&content)?;
        Ok(bot_cfg)
      })
    })
    .map_or_else(
      |_| {
        write_default_bot_config();
        let bot_cfg = serde_json::from_str::<BotCfg>(include_str!(concat!(
          env!("CARGO_MANIFEST_DIR"),
          "/..",
          "/config/bot.json"
        )))
        .expect("can't parse default bot.json");
        bot_cfg
      },
      |bots| bots,
    )
}

fn bot_vars_parser(file: &str) -> Option<Vec<Bot>> {
  let bot_cfg = serde_json::from_str::<serde_json::Value>(file);
  match bot_cfg {
    Ok(bot_cfg) => {
      let vars = bot_cfg.get("vars").map(|vars| {
        serde_json::from_value::<HashMap<String, String>>(vars.clone()).expect("can't parse vars")
      });

      if let Some(vars) = vars {
        let bots_str = String::from(file);
        let replaced_str = vars.iter().fold(bots_str, |str, (key, value)| {
          let regex = Regex::new(&format!(r"([^$])\{{{}\}}", key)).unwrap();
          println!("regex: {:?}", regex);
          println!("value: {}", value);
          regex
            .replace_all(&str, format!("${{1}}{}", value))
            .to_string()
        });
        println!("replaced_str: {}", replaced_str);
        let ret = serde_json::from_str::<BotCfg>(&replaced_str);
        match ret {
          Ok(data) => Some(data.bots),
          Err(err) => {
            println!("bot.json parse error: {}", err);
            None
          }
        }
      } else {
        serde_json::from_str::<BotCfg>(file)
          .ok()
          .map(|bot_cfg| bot_cfg.bots)
      }
    }
    Err(err) => {
      println!("bot.json parse error: {}", err);
      None
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn vars_replace_parser() {
    let file = r#"{
      "vars": {
        "token": "abc"
      },
      "bots": [
        {
          "id": "87cc7e88-fd55-4c01-9a34-d0f1397b1c73",
          "name": "Polestar Assistant",
          "desc": "Polestar Assistant is a AI assistant powered by OpenAI",
          "avatar": {
            "name": "🌉",
            "color": "EDF7FBFF"
          },
          "cat": "Assistant",
          "tags": [
            "AI"
          ],
          "lang": [
            "en"
          ],
          "sp": "OpenAI",
          "url": "https://api.openai.com/v1/chat/completions",
          "headers": {
            "Authorization": "{token}"
          },
          "params": {
            "model": "gpt-3.5-turbo",
            "prompt": ""
          }
        },
        {
          "id": "87cc7e88-fd55-4c01-9a34-d0f1397b1c73",
          "name": "Polestar Assistant",
          "desc": "Polestar Assistant is a AI assistant powered by OpenAI",
          "avatar": {
            "name": "🌉",
            "color": "EDF7FBFF"
          },
          "cat": "Assistant",
          "tags": [
            "AI"
          ],
          "lang": [
            "en"
          ],
          "sp": "OpenAI",
          "url": "https://api.openai.com/v1/chat/completions",
          "headers": {
            "Authorization": "${token}"
          },
          "params": {
            "model": "gpt-3.5-turbo",
            "prompt": ""
          }
        }
      ]
    }"#;
    let bots = bot_vars_parser(file);
    let bots = bots.expect("can't parse bots");
    assert_eq!(bots.len(), 2);
    assert_eq!(
      bots[0].headers().get("Authorization"),
      Some(&String::from("abc"))
    );
  }
}
