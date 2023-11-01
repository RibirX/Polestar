use std::ptr::NonNull;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::pool::PersistenceDB;

use super::msg::Msg;

#[derive(Debug, Clone)]
pub struct Channel {
  id: Uuid,
  name: String,
  desc: Option<String>,
  cfg: ChannelCfg,
  msgs_coll: MsgColl,
  db: Option<NonNull<PersistenceDB>>,
}

#[derive(Debug, Default, Clone)]
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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum MsgsCollStatus {
  #[default]
  NonFetched,
  Fetched,
}

impl Channel {
  pub fn new(id: Uuid, name: String, desc: Option<String>, cfg: ChannelCfg, db: Option<NonNull<PersistenceDB>>) -> Self {
    Self {
      id,
      name,
      desc,
      cfg,
      msgs_coll: MsgColl::default(),
      db,
    }
  }

  #[inline]
  pub fn set_db(&mut self, db: NonNull<PersistenceDB>) { self.db = Some(db); }

  #[inline]
  pub fn id(&self) -> &Uuid { &self.id }

  #[inline]
  pub fn name(&self) -> &str { &self.name }

  #[inline]
  pub fn desc(&self) -> Option<&str> { self.desc.as_deref() }

  #[inline]
  pub fn cfg(&self) -> &ChannelCfg { &self.cfg }

  #[inline]
  pub fn msgs(&self) -> &Vec<Msg> { &self.msgs_coll.msgs }

  #[inline]
  pub fn update_name(&mut self, name: String) { self.name = name; }

  #[inline]
  pub fn update_desc(&mut self, desc: Option<String>) { self.desc = desc; }

  #[inline]
  pub fn cfg_mut(&mut self) -> &mut ChannelCfg { &mut self.cfg }

  #[inline]
  pub fn add_msg(&mut self, msg: Msg) { self.msgs_coll.msgs.push(msg); }

  pub fn loaded_msgs_from_db(&mut self, msgs: Vec<Msg>) {
    if self.msgs_coll.status == MsgsCollStatus::NonFetched {
      self.msgs_coll.msgs = msgs;
      self.msgs_coll.status = MsgsCollStatus::Fetched;
    } else {
      // TODO: log warning.
    }
  }

  // this method is used to get a message and update it.
  pub fn msg_mut(&mut self, msg_id: &Uuid) -> Option<&mut Msg> {
    self
      .msgs_coll
      .msgs
      .iter_mut()
      .find(|msg| msg.id() == msg_id)
  }
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct ChannelCfg {
  mode: ChannelMode,
  kind: ChannelKind,
  // if channel default bot id is none, it means this channel use global default bot id.
  def_bot_id: Option<Uuid>,
}

impl ChannelCfg {
  #[inline]
  pub fn mode(&self) -> ChannelMode { self.mode }

  #[inline]
  pub fn kind(&self) -> ChannelKind { self.kind }

  #[inline]
  pub fn def_bot_id(&self) -> Option<&Uuid> { self.def_bot_id.as_ref() }

  #[inline]
  pub fn set_mode(&mut self, mode: ChannelMode) { self.mode = mode; }

  #[inline]
  pub fn set_kind(&mut self, kind: ChannelKind) { self.kind = kind; }

  #[inline]
  pub fn set_def_bot_id(&mut self, def_bot_id: Option<Uuid>) { self.def_bot_id = def_bot_id; }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, Default)]
pub enum ChannelMode {
  #[default]
  Balanced,
  Performance,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, Default)]
pub enum ChannelKind {
  #[default]
  Chat,
  QuickLauncher,
  Feedback,
}
