use derive_builder::Builder;
use serde::Deserialize;

use crate::service::req::request_quota;

use super::GLOBAL_VARS;

#[derive(Builder, Debug, Default, Clone)]
#[builder(name = "UserBuilder")]
#[builder(pattern = "owned")]
#[builder(setter(into, strip_option), default)]
pub struct User {
  token: Option<String>,
  nick_name: Option<String>,
  email: Option<String>,
  uid: u64,
  quota: Option<Quota>,
}

impl User {
  #[inline]
  pub fn token(&self) -> Option<&str> { self.token.as_deref() }

  #[inline]
  pub fn set_token(&mut self, token: Option<String>) {
    self.token = token.clone();
    if let Some(token) = token {
      GLOBAL_VARS
        .try_lock()
        .unwrap()
        .insert(super::GlbVar::PolestarKey, token);
    }
  }

  #[inline]
  pub fn nick_name(&self) -> Option<&String> { self.nick_name.as_ref() }

  #[inline]
  pub fn set_nick_name(&mut self, nick_name: Option<String>) { self.nick_name = nick_name; }

  #[inline]
  pub fn email(&self) -> Option<&String> { self.email.as_ref() }

  #[inline]
  pub fn set_email(&mut self, email: Option<String>) { self.email = email; }

  #[inline]
  pub fn uid(&self) -> u64 { self.uid }

  pub fn set_quota(&mut self, quota: Option<Quota>) { self.quota = quota; }

  #[inline]
  pub fn quota(&self) -> Option<&Quota> { self.quota.as_ref() }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Quota {
  picture_used: f32,
  text_used: f32,
  max_texts: f32,
  max_pictures: f32,
  request_cnt: f32,
}

impl Quota {
  #[inline]
  pub fn picture_used(&self) -> f32 { self.picture_used }

  #[inline]
  pub fn text_used(&self) -> f32 { self.text_used }

  #[inline]
  pub fn max_texts(&self) -> f32 { self.max_texts }

  #[inline]
  pub fn max_pictures(&self) -> f32 { self.max_pictures }

  #[inline]
  pub fn request_cnt(&self) -> f32 { self.request_cnt }

  pub fn is_over(&self) -> bool {
    self.picture_used > self.max_pictures || self.text_used > self.max_texts
  }
}

#[derive(Deserialize, Debug)]
pub struct QuotaInfo {
  user_id: u64,
  limits: f32,
  used: f32,
  pub statistics: serde_json::Value,
}
