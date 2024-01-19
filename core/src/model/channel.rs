use std::ptr::NonNull;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::{executor::ActionPersist, pool::PersistenceDB};

use super::{msg::Msg, AppInfo, Bot, BotId, MsgAction};

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

  pub fn set_cfg(&mut self, cfg: ChannelCfg) {
    self.cfg = cfg;
    self.update();
  }

  #[inline]
  pub fn set_name(&mut self, name: String) {
    self.name = name;
    self.update();
  }

  #[inline]
  pub fn set_desc(&mut self, desc: Option<String>) {
    self.desc = desc;
    self.update();
  }

  pub fn add_msg(&mut self, msg: Msg) {
    self.msgs_coll.msgs.push(msg.clone());
    let channel_id = *self.id();
    if let Some(db) = self.db.as_mut() {
      unsafe {
        db.as_mut()
          .persist_async(ActionPersist::AddMsg { channel_id, msg })
      }
    }
  }

  pub fn update_msg(&mut self, msg_id: &Uuid, idx: usize, act: MsgAction) {
    let msg = self.msg_mut(msg_id);
    if let Some(msg) = msg {
      let is_need_persist = matches!(act, MsgAction::Fulfilled | MsgAction::Rejected);
      msg.cont_mut(idx).action(act);
      if is_need_persist {
        let msg = msg.clone();
        if let Some(db) = self.db.as_mut() {
          unsafe { db.as_mut().persist_async(ActionPersist::UpdateMsg { msg }) }
        }
      }
    }
  }

  pub fn load_msgs(&mut self, msgs: Vec<Msg>) {
    self.msgs_coll.msgs.extend(msgs.clone());
    self.msgs_coll.status = MsgsCollStatus::Fetched;
  }

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

  pub fn bots(&self) -> Option<&[Bot]> { self.app_info().map(|info| info.bots()) }

  pub fn is_feedback(&self) -> bool { self.cfg.kind == ChannelKind::Feedback }

  fn update(&mut self) {
    if let Some(db) = self.db.as_mut() {
      unsafe {
        db.as_mut().persist_async(ActionPersist::UpdateChannel {
          id: *self.id(),
          name: self.name.clone(),
          desc: self.desc.clone(),
          cfg: self.cfg.clone(),
        })
      }
    }
  }
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct ChannelCfg {
  mode: ChannelMode,
  kind: ChannelKind,
  // if channel default bot id is none, it means this channel use global default bot id.
  def_bot_id: Option<BotId>,
}

impl ChannelCfg {
  pub fn new(mode: ChannelMode, kind: ChannelKind, def_bot_id: Option<BotId>) -> Self {
    Self { mode, kind, def_bot_id }
  }

  pub fn feedback_cfg() -> Self {
    Self {
      kind: ChannelKind::Feedback,
      ..<_>::default()
    }
  }

  pub fn def_bot_id_cfg(bot_id: BotId) -> Self {
    Self {
      def_bot_id: Some(bot_id),
      ..<_>::default()
    }
  }

  #[inline]
  pub fn mode(&self) -> ChannelMode { self.mode }

  #[inline]
  pub fn def_bot_id(&self) -> Option<&BotId> { self.def_bot_id.as_ref() }

  #[inline]
  pub fn set_mode(&mut self, mode: ChannelMode) { self.mode = mode; }

  #[inline]
  pub fn set_def_bot_id(&mut self, def_bot_id: Option<BotId>) { self.def_bot_id = def_bot_id; }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, Default)]
pub enum ChannelKind {
  #[default]
  #[serde(rename = "chat")]
  Chat,
  #[serde(rename = "feedback")]
  Feedback,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, Default)]
pub enum ChannelMode {
  #[default]
  #[serde(rename = "balanced")]
  Balanced,
  #[serde(rename = "performance")]
  Performance,
}

impl ChannelMode {
  pub fn context_number(&self) -> usize {
    match self {
      ChannelMode::Balanced => 6,
      ChannelMode::Performance => 10,
    }
  }
}
