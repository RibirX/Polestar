use std::{collections::HashMap, fs, io::Read, path::PathBuf};

use log::warn;
use regex::Regex;
use serde::Deserialize;

use crate::{
  error::PolestarResult,
  launch::write_default_bot_config,
  model::{Bot, BotId, PartialBot, ServerProvider},
  project_config_path, user_cfg_path, user_data_path,
};

#[derive(Deserialize, Debug)]
struct BotFileCfg {
  bots: Option<Vec<Bot>>,
  providers: Option<Vec<ServerProvider>>,
}

#[derive(Deserialize, Debug)]
struct PartialBotFileCfg {
  bots: Option<Vec<PartialBot>>,
  providers: Option<Vec<ServerProvider>>,
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

pub fn load_bot_cfg(uid: &str) -> PolestarResult<BotCfg> {
  write_default_bot_config();

  let user_cfg_path = user_cfg_path(uid);
  fs::File::open(user_cfg_path)
    .and_then(|mut file| {
      let mut content = String::new();
      file
        .read_to_string(&mut content)
        .map(|_| parse_user_bot_cfgs(user_data_path(uid), &content))
    })
    .unwrap_or_else(|_| {
      let content = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/..",
        "/config/bot.json"
      ));
      let BotFileCfg { bots, providers } = parse_bot_config(content)?;
      Ok(BotCfg {
        bots: bots.unwrap_or_default(),
        providers: providers
          .unwrap_or_default()
          .into_iter()
          .map(|sp| (sp.name.clone(), sp))
          .collect(),
      })
    })
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

fn parse_user_bot_cfgs(user_data_path: PathBuf, file: &str) -> PolestarResult<BotCfg> {
  let user_file_cfg = serde_json::from_str::<UserFileCfg>(file)?;
  let mut user_partial_bots = vec![];
  let mut user_sp = HashMap::new();
  if let Some(files) = user_file_cfg.files {
    for file in files {
      let PartialBotFileCfg { bots, providers } =
        parse_partial_bot_config_file(&user_data_path.join(file))?;
      if let Some(bots) = bots {
        user_partial_bots.extend(bots.into_iter());
      }
      if let Some(sp) = providers {
        sp.into_iter().for_each(|sp| {
          if let Some(old_val) = user_sp.insert(sp.name.clone(), sp) {
            warn!(
              "Server provider {} is duplicated, the old one will be replaced by the new one",
              old_val.name
            );
          };
        })
      }
    }
  }

  let mut official_bots = vec![];
  if let Some(base) = user_file_cfg.base {
    let BotFileCfg { bots, providers } = parse_bot_config_file(&user_data_path.join(base.extends))?;

    if let Some(bots) = bots {
      official_bots.extend(
        bots
          .into_iter()
          .filter(|bot| need_bot(bot, base.includes.as_ref(), base.excludes.as_ref())),
      );
    }

    if let Some(sp) = providers {
      sp.into_iter().for_each(|sp| {
        if let Some(old_val) = user_sp.insert(sp.name.clone(), sp) {
          warn!(
            "Server provider {} is conflicted with official one, the official one will be replaced by the new one",
            old_val.name
          );
        };
      })
    }
  }

  let mut remove_idx = vec![];
  official_bots.iter().for_each(|bot| {
    if let Some(idx) = user_partial_bots.iter().position(|b| b.id() == bot.id()) {
      remove_idx.push(idx);
    }
  });

  let mut user_bots = vec![];

  for idx in remove_idx.into_iter().rev() {
    let bot = user_partial_bots.remove(idx);
    official_bots
      .iter_mut()
      .find(|b| b.id() == bot.id())
      .map(|b| b.merge(&bot));
  }

  user_bots.extend(user_partial_bots.into_iter().filter_map(|bot| bot.to_bot()));

  Ok(BotCfg {
    bots: official_bots
      .into_iter()
      .chain(user_bots.into_iter())
      .collect(),
    providers: user_sp,
  })
}

#[derive(Deserialize, Debug)]
struct CfgVars {
  vars: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Debug)]
pub struct BotCfg {
  pub bots: Vec<Bot>,
  pub providers: HashMap<String, ServerProvider>,
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

fn parse_bot_config_file(file: &PathBuf) -> PolestarResult<BotFileCfg> {
  let mut file = fs::File::open(file)?;
  let mut content = String::new();
  file.read_to_string(&mut content)?;
  parse_bot_config(&content)
}

fn parse_bot_config(file: &str) -> PolestarResult<BotFileCfg> {
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
    serde_json::from_str::<BotFileCfg>(&replaced_str).map_err(|err| err.into())
  } else {
    serde_json::from_str::<BotFileCfg>(file).map_err(|err| err.into())
  }
}

fn parse_partial_bot_config_file(file: &PathBuf) -> PolestarResult<PartialBotFileCfg> {
  let mut file = fs::File::open(file)?;
  let mut content = String::new();
  file.read_to_string(&mut content)?;
  parse_partial_bot_config(&content)
}

fn parse_partial_bot_config(file: &str) -> PolestarResult<PartialBotFileCfg> {
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
    serde_json::from_str::<PartialBotFileCfg>(&replaced_str).map_err(|err| err.into())
  } else {
    serde_json::from_str::<PartialBotFileCfg>(file).map_err(|err| err.into())
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
      "providers": [
        {
          "name": "OpenAI",
          "base_url": "https://api.openai.com/v1/chat/completions",
          "token": "bear {token}"
        }
      ],
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

    let cfg = parse_bot_config(file).expect("can't parse bots config");
    let bots = cfg.bots.as_ref().unwrap();
    let providers = cfg.providers.as_ref().unwrap();
    assert_eq!(bots.len(), 2);
    assert_eq!(providers.len(), 1);
    assert_eq!(providers[0].token, "bear abc");
    assert_eq!(
      bots[0].headers().get("Authorization"),
      Some(&String::from("abc"))
    );
    assert_eq!(
      bots[1].headers().get("Authorization"),
      Some(&String::from("${token}"))
    );
  }

  #[test]
  fn test() {
    let reg = Regex::new(r"\{\s*([^}]*)\s*\}").unwrap();
    let caps = reg.captures_iter("123{123}{}");
    for cap in caps {
      println!("{}", &cap[1]);
    }
  }
}
