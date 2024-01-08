use std::{collections::HashMap, fs, io::Read, path::PathBuf};

use regex::Regex;
use serde::Deserialize;

use crate::{
  error::PolestarResult,
  launch::write_default_bot_config,
  model::{Bot, BotId},
  project_config_path, user_cfg_path, user_data_path,
};

#[derive(Deserialize, Debug)]
struct BotFileCfg {
  vars: Option<HashMap<String, String>>,
  bots: Option<Vec<Bot>>,
}

#[derive(Deserialize, Debug)]
struct UserFileCfg {
  base: Option<UserFileBase>,
  files: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
struct UserFileBase {
  extends: String,
  includes: Option<Vec<BotId>>,
  excludes: Option<Vec<BotId>>,
}

pub fn load_bot_cfg(uid: &str) -> PolestarResult<Vec<Bot>> {
  let user_cfg_path = user_cfg_path(uid);

  fs::File::open(user_cfg_path)
    .and_then(|mut file| {
      let mut content = String::new();
      file
        .read_to_string(&mut content)
        .map(|_| user_bot_cfg_parser(user_data_path(uid), &content))
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

fn need_bot(bot: &Bot, includes: Option<&Vec<BotId>>, excludes: Option<&Vec<BotId>>) -> bool {
  if let Some(includes) = includes {
    includes.iter().any(|id| id == bot.id())
  } else if let Some(excludes) = excludes {
    excludes.iter().all(|id| id != bot.id())
  } else {
    true
  }
}

fn user_bot_cfg_parser(user_data_path: PathBuf, file: &str) -> PolestarResult<Vec<Bot>> {
  let user_file_cfg = serde_json::from_str::<UserFileCfg>(file)?;
  let mut user_bots = vec![];
  if let Some(files) = user_file_cfg.files {
    for file in files {
      let mut file = fs::File::open(user_data_path.join(file))?;
      let mut content = String::new();
      file.read_to_string(&mut content)?;
      user_bots.extend(bot_vars_parser(&content)?);
    }
  }

  let mut official_bots = vec![];
  if let Some(base) = user_file_cfg.base {
    let mut base_file = fs::File::open(user_data_path.join(base.extends))?;
    let mut base_content = String::new();
    base_file.read_to_string(&mut base_content)?;
    official_bots.extend(bot_vars_parser(&base_content)?.into_iter().filter(|bot| {
      user_bots.iter().all(|user_bot| user_bot.id() != bot.id())
        || need_bot(bot, base.includes.as_ref(), base.excludes.as_ref())
    }));
  }

  Ok(
    official_bots
      .into_iter()
      .chain(user_bots.into_iter())
      .collect(),
  )
}

#[derive(Deserialize, Debug)]
struct BotCfg {
  bots: Vec<Bot>,
}

#[derive(Deserialize, Debug)]
struct CfgVars {
  vars: Option<HashMap<String, String>>,
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
