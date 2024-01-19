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

  sp: String,
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
  sp: Option<String>,
  url: Option<String>,
  headers: Option<HashMap<String, String>>,
  params: Option<serde_json::Value>,
  onboarding: Option<String>,
}

impl PartialBot {
  pub fn id(&self) -> &BotId { &self.id }

  pub fn to_bot(self) -> Option<Bot> {
    self.is_complete().then(|| Bot {
      id: self.id,
      name: self.name.unwrap(),
      lang: self.lang.unwrap(),
      desc: self.desc,
      avatar: self.avatar.unwrap(),
      cat: self.cat,
      tags: self.tags.unwrap(),
      sp: self.sp.unwrap(),
      url: self.url.unwrap(),
      headers: self.headers.unwrap(),
      params: self.params.unwrap(),
      onboarding: self.onboarding,
    })
  }

  fn is_complete(&self) -> bool {
    self.name.is_some()
      && self.lang.is_some()
      && self.avatar.is_some()
      && self.tags.is_some()
      && self.sp.is_some()
      && self.url.is_some()
      && self.headers.is_some()
      && self.params.is_some()
  }
}

impl Bot {
  pub fn id(&self) -> &BotId { &self.id }

  pub fn name(&self) -> &str { &self.name }

  pub fn desc(&self) -> Option<&str> { self.desc.as_deref() }

  pub fn avatar(&self) -> &BotAvatar { &self.avatar }

  pub fn cat(&self) -> Option<&str> { self.cat.as_deref() }

  pub fn tags(&self) -> &[String] { &self.tags }

  pub fn headers(&self) -> &HashMap<String, String> { &self.headers }

  pub fn sp(&self) -> &str { &self.sp }

  pub fn url(&self) -> &str { &self.url }

  pub fn params(&self) -> &serde_json::Value { &self.params }

  pub fn lang(&self) -> &[Lang] { &self.lang }

  pub fn onboarding(&self) -> Option<&str> { self.onboarding.as_deref() }

  pub fn merge(&mut self, bot: &PartialBot) {
    if let Some(name) = &bot.name {
      self.name = name.clone();
    }
    if let Some(lang) = &bot.lang {
      self.lang = lang.clone();
    }
    if let Some(desc) = &bot.desc {
      self.desc = Some(desc.clone());
    }
    if let Some(avatar) = &bot.avatar {
      self.avatar = avatar.clone();
    }
    if let Some(cat) = &bot.cat {
      self.cat = Some(cat.clone());
    }
    if let Some(tags) = &bot.tags {
      self.tags = tags.clone();
    }
    if let Some(sp) = &bot.sp {
      self.sp = sp.clone();
    }
    if let Some(url) = &bot.url {
      self.url = url.clone();
    }
    if let Some(headers) = &bot.headers {
      self.headers = headers.clone();
    }
    if let Some(params) = &bot.params {
      self.params = params.clone();
    }
    if let Some(onboarding) = &bot.onboarding {
      self.onboarding = Some(onboarding.clone());
    }
  }
}

#[derive(Deserialize, Debug, Clone, Copy)]
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
