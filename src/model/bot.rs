use std::collections::HashMap;

use serde::Deserialize;
use uuid::Uuid;

/// Bot is Polestar basic component, user need talk to bot to get AI response.
#[derive(Deserialize, Debug)]
pub struct Bot {
  // A unique id for bot
  id: Uuid,
  // A name for bot
  name: String,
  // This bot support languages version
  lang: Vec<Lang>,
  // A description for bot, it's optional
  desc: Option<String>,
  // A category for bot, it's optional
  cat: Option<String>,
  // A list of tags for bot, it can be empty list to indicate no tags
  tags: Vec<String>,
  // API url for bot's AI service
  url: String,
  // API url header for bot's AI service
  headers: HashMap<String, String>,
  // The `sp` field indicate AI service model need parameters
  params: serde_json::Value,
}

impl Bot {
  pub fn id(&self) -> &Uuid { &self.id }

  pub fn name(&self) -> &str { &self.name }

  pub fn desc(&self) -> Option<&str> { self.desc.as_deref() }

  pub fn cat(&self) -> Option<&str> { self.cat.as_deref() }

  pub fn tags(&self) -> &[String] { &self.tags }

  pub fn headers(&self) -> &HashMap<String, String> { &self.headers }

  pub fn url(&self) -> &str { &self.url }

  pub fn params(&self) -> &serde_json::Value { &self.params }

  pub fn lang(&self) -> &[Lang] { &self.lang }
}

#[derive(Deserialize, Debug)]
pub enum Lang {
  #[serde(rename = "en")]
  En,
  #[serde(rename = "zh-CN")]
  ZhCN,
}

#[cfg(test)]
mod test {
  use super::*;

  // load bot.json file
  #[test]
  fn load_bot_cfg_file() {
    let preset_bot_cfg =
      serde_json::from_str::<serde_json::Value>(include_str!("../../config/bot.json"))
        .expect("Failed to parse bot.json");
    let bots = serde_json::from_value::<Vec<Bot>>(preset_bot_cfg["bots"].clone())
      .expect("Failed to parse bots");
    let bot = bots.first();
    println!("bot: {:?}", bot);
  }
}
