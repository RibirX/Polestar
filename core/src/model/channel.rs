use std::ptr::NonNull;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::pool::PersistenceDB;

use super::{msg::Msg, AppInfo, Bot};

pub type ChannelId = Uuid;

#[derive(Debug, Clone, Deserialize)]
pub struct Channel {
  id: ChannelId,
  name: String,
  desc: Option<String>,
  cfg: ChannelCfg,
  msgs_coll: MsgColl,
  #[serde(skip)]
  app_info: Option<NonNull<AppInfo>>,
  #[serde(skip)]
  db: Option<NonNull<PersistenceDB>>,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct MsgColl {
  msgs: Vec<Msg>,
  status: MsgsCollStatus,
}

impl MsgColl {
  #[inline]
  pub fn msgs(&self) -> &Vec<Msg> { &self.msgs }

  #[inline]
  pub fn status(&self) -> MsgsCollStatus { self.status }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default, Deserialize)]
pub enum MsgsCollStatus {
  #[default]
  #[serde(rename = "non_fetched")]
  NonFetched,
  #[serde(rename = "fetched")]
  Fetched,
}

impl Channel {
  pub fn new(
    id: ChannelId,
    name: String,
    desc: Option<String>,
    cfg: ChannelCfg,
    app_info: Option<NonNull<AppInfo>>,
    db: Option<NonNull<PersistenceDB>>,
  ) -> Self {
    Self {
      id,
      name,
      desc,
      cfg,
      msgs_coll: MsgColl::default(),
      app_info,
      db,
    }
  }

  #[inline]
  pub fn set_db(&mut self, db: NonNull<PersistenceDB>) { self.db = Some(db); }

  #[inline]
  pub fn set_app_info(&mut self, app_info: NonNull<AppInfo>) { self.app_info = Some(app_info); }

  pub fn app_info(&self) -> Option<&AppInfo> {
    self.app_info.map(|app_info| unsafe { app_info.as_ref() })
  }

  #[inline]
  pub fn id(&self) -> &ChannelId { &self.id }

  #[inline]
  pub fn name(&self) -> &str { &self.name }

  #[inline]
  pub fn desc(&self) -> Option<&str> { self.desc.as_deref() }

  #[inline]
  pub fn cfg(&self) -> &ChannelCfg { &self.cfg }

  #[inline]
  pub fn cfg_mut(&mut self) -> &mut ChannelCfg { &mut self.cfg }

  #[inline]
  pub fn set_name(&mut self, name: String) { self.name = name; }

  #[inline]
  pub fn set_desc(&mut self, desc: Option<String>) { self.desc = desc; }

  #[inline]
  pub fn add_msg(&mut self, msg: Msg) { self.msgs_coll.msgs.push(msg); }

  #[inline]
  pub fn msgs(&self) -> &Vec<Msg> { &self.msgs_coll.msgs }

  #[inline]
  pub fn msgs_mut(&mut self) -> &mut Vec<Msg> { &mut self.msgs_coll.msgs }

  pub fn msg(&self, msg_id: &Uuid) -> Option<&Msg> {
    self.msgs_coll.msgs.iter().find(|msg| msg.id() == msg_id)
  }

  pub fn last_msg(&self) -> Option<&Msg> { self.msgs_coll.msgs.last() }

  // this method is used to get a message and update it.
  pub fn msg_mut(&mut self, msg_id: &Uuid) -> Option<&mut Msg> {
    self
      .msgs_coll
      .msgs
      .iter_mut()
      .find(|msg| msg.id() == msg_id)
  }

  pub fn bots(&self) -> Option<&Vec<Bot>> { self.app_info().map(|info| info.bots()) }
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct ChannelCfg {
  mode: ChannelMode,
  // if channel default bot id is none, it means this channel use global default bot id.
  def_bot_id: Option<Uuid>,
}

impl ChannelCfg {
  #[inline]
  pub fn mode(&self) -> ChannelMode { self.mode }

  #[inline]
  pub fn def_bot_id(&self) -> Option<&Uuid> { self.def_bot_id.as_ref() }

  #[inline]
  pub fn set_mode(&mut self, mode: ChannelMode) { self.mode = mode; }

  #[inline]
  pub fn set_def_bot_id(&mut self, def_bot_id: Option<Uuid>) { self.def_bot_id = def_bot_id; }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, Default)]
pub enum ChannelMode {
  #[default]
  #[serde(rename = "balanced")]
  Balanced,
  #[serde(rename = "performance")]
  Performance,
}
