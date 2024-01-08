use std::collections::HashMap;

use regex::Regex;
use serde::Deserialize;

use crate::{error::PolestarResult, model::Bot, project_bot_config_path, project_config_path};

#[derive(Deserialize, Debug)]
struct BotCfg {
  bots: Vec<Bot>,
}

#[derive(Deserialize, Debug)]
struct CfgVars {
  vars: Option<HashMap<String, String>>,
}

pub fn load_bot_cfg_file() -> PolestarResult<Vec<Bot>> {
  use super::launch::write_default_bot_config;
  use std::{fs, io::Read};

  let path = project_bot_config_path();

  fs::File::open(path)
    .and_then(|mut file| {
      let mut content = String::new();
      file
        .read_to_string(&mut content)
        .map(|_| bot_vars_parser(&content))
    })
    .map_or_else(
      |_| {
        write_default_bot_config();
        let content = include_str!(concat!(
          env!("CARGO_MANIFEST_DIR"),
          "/..",
          "/config/bot.json"
        ));
        bot_vars_parser(content)
      },
      |bots| bots,
    )
}

pub fn open_user_config_folder() {
  use std::process::Command;
  #[cfg(target_os = "macos")]
  {
    Command::new("open")
      .arg(project_config_path()) // <- Specify the directory you'd like to open.
      .spawn()
      .unwrap();
  }
  #[cfg(target_os = "windows")]
  {
    Command::new("explorer")
      .arg(project_config_path()) // <- Specify the directory you'd like to open.
      .spawn()
      .unwrap();
  }
  #[cfg(target_os = "linux")]
  {
    Command::new("xdg-open")
      .arg(project_config_path()) // <- Specify the directory you'd like to open.
      .spawn()
      .unwrap();
  }
}

fn bot_vars_parser(file: &str) -> PolestarResult<Vec<Bot>> {
  if let Some(vars) = serde_json::from_str::<CfgVars>(file)?.vars {
    let bots_str = String::from(file);
    let replaced_str = vars.iter().fold(bots_str, |str, (key, value)| {
      // ${xx} is dynamic value which will be replaced in runtime
      // only {xx} will be replaced to correspond vars when parsing
      let regex = Regex::new(&format!(r"([^$])\{{{}\}}", key)).unwrap();
      regex
        .replace_all(&str, format!("${{1}}{}", value))
        .to_string()
    });
    serde_json::from_str::<BotCfg>(&replaced_str)
      .map(|bot_cfg| bot_cfg.bots)
      .map_err(|err| err.into())
  } else {
    serde_json::from_str::<BotCfg>(file)
      .map(|bot_cfg| bot_cfg.bots)
      .map_err(|err| err.into())
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
    assert_eq!(
      bots[1].headers().get("Authorization"),
      Some(&String::from("${token}"))
    );
  }
}
