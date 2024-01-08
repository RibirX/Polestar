use serde::Deserialize;
use std::collections::HashMap;

pub type BotId = String;

/// Bot is Polestar basic widget, user need talk to bot to get AI response.
#[derive(Deserialize, Debug)]
pub struct Bot {
  // A unique id for bot
  id: BotId,
  // A name for bot
  name: String,
  // This bot support languages version
  lang: Vec<Lang>,
  // A description for bot, it's optional
  desc: Option<String>,
  // A avatar for bot
  avatar: BotAvatar,
  // A category for bot, it's optional
  cat: Option<String>,
  // A list of tags for bot, it can be empty list to indicate no tags
  tags: Vec<String>,
  sp: ServiceModel,
  // API url for bot's AI service
  url: String,
  // API url header for bot's AI service
  headers: HashMap<String, String>,
  // The `sp` field indicate AI service model need parameters
  params: serde_json::Value,
  // The bot's onboarding message, it's optional
  onboarding: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct PartialBot {
  id: BotId,
  name: Option<String>,
  lang: Option<Vec<Lang>>,
  desc: Option<String>,
  avatar: Option<BotAvatar>,
  cat: Option<String>,
  tags: Option<Vec<String>>,
  sp: Option<ServiceModel>,
  url: Option<String>,
  headers: Option<HashMap<String, String>>,
  params: Option<serde_json::Value>,
  onboarding: Option<String>,
}

impl Bot {
  pub fn id(&self) -> &BotId { &self.id }

  pub fn name(&self) -> &str { &self.name }

  pub fn desc(&self) -> Option<&str> { self.desc.as_deref() }

  pub fn avatar(&self) -> &BotAvatar { &self.avatar }

  pub fn cat(&self) -> Option<&str> { self.cat.as_deref() }

  pub fn tags(&self) -> &[String] { &self.tags }

  pub fn headers(&self) -> &HashMap<String, String> { &self.headers }

  pub fn sp(&self) -> &ServiceModel { &self.sp }

  pub fn url(&self) -> &str { &self.url }

  pub fn params(&self) -> &serde_json::Value { &self.params }

  pub fn lang(&self) -> &[Lang] { &self.lang }

  pub fn onboarding(&self) -> Option<&str> { self.onboarding.as_deref() }
}

#[derive(Deserialize, Debug)]
pub enum Lang {
  #[serde(rename = "en")]
  En,
  #[serde(rename = "zh-CN")]
  ZhCN,
}

#[derive(Clone, Deserialize, Debug)]
#[serde(untagged)]
pub enum BotAvatar {
  Text { name: String, color: String },
  Image { url: String },
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceModel {
  OpenAI,
}
