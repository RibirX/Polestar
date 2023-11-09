use std::{pin::Pin, ptr::NonNull};

use uuid::Uuid;

use crate::db::{executor::ActionPersist, pool::PersistenceDB};

use super::{
  bot::Bot,
  channel::{Channel, ChannelCfg},
};

pub struct AppData {
  bots: Vec<Bot>,
  channels: Vec<Channel>,
  cur_channel_id: Uuid,
  cfg: AppCfg,
  db: Option<Pin<Box<PersistenceDB>>>,
}

impl AppData {
  pub fn new(
    bots: Vec<Bot>,
    channels: Vec<Channel>,
    def_bot_id: Uuid,
    db: Option<Pin<Box<PersistenceDB>>>,
  ) -> Self {
    // TODO: record User last current channel id in local file.
    let cur_channel_id = channels.first().map(|c| *c.id()).unwrap_or(Uuid::nil());
    Self {
      bots,
      channels,
      cur_channel_id,
      cfg: AppCfg::new(None, def_bot_id),
      db,
    }
  }

  #[inline]
  pub fn cur_channel_id(&self) -> &Uuid { &self.cur_channel_id }

  #[inline]
  pub fn bots(&self) -> &Vec<Bot> { &self.bots }

  #[inline]
  pub fn channels(&self) -> &Vec<Channel> { &self.channels }

  #[inline]
  pub fn channels_mut(&mut self) -> &mut Vec<Channel> { &mut self.channels }

  pub fn get_channel(&self, channel_id: &Uuid) -> Option<&Channel> {
    self
      .channels
      .iter()
      .find(|channel| channel.id() == channel_id)
  }

  pub fn get_channel_mut(&mut self, channel_id: &Uuid) -> Option<&mut Channel> {
    self
      .channels
      .iter_mut()
      .find(|channel| channel.id() == channel_id)
  }

  pub fn switch_channel(&mut self, channel_id: &Uuid) { self.cur_channel_id = *channel_id; }

  pub fn cur_channel(&self) -> Option<&Channel> {
    self
      .channels
      .iter()
      .find(|channel| channel.id() == &self.cur_channel_id)
  }

  pub fn cur_channel_mut(&mut self) -> Option<&mut Channel> {
    self
      .channels
      .iter_mut()
      .find(|channel| channel.id() == &self.cur_channel_id)
  }

  pub fn new_channel(&mut self, name: String, desc: Option<String>) -> Uuid {
    let id = Uuid::new_v4();
    let db = self.db.as_mut().map(|db| NonNull::from(&**db));
    let channel = Channel::new(id, name, desc, ChannelCfg::default(), db);

    let p_channel = channel.clone();

    self.db.as_mut().map(|db| {
      db.as_mut()
        .add_persist(ActionPersist::AddChannel { channel: p_channel.clone() })
    });

    self.channels.push(channel);
    self.cur_channel_id = id;

    id
  }

  pub fn remove_channel(&mut self, channel_id: &Uuid) {
    let p_channel_id = *channel_id;

    self.db.as_mut().map(|db| {
      db.as_mut()
        .add_persist(ActionPersist::RemoveChannel { channel_id: p_channel_id })
    });

    self.channels.retain(|channel| channel.id() != channel_id);
  }

  pub fn cfg(&self) -> &AppCfg { &self.cfg }

  pub fn cfg_mut(&mut self) -> &mut AppCfg { &mut self.cfg }

  pub fn clear_channels(&mut self) {
    let channel_ids: Vec<Uuid> = self.channels.iter().map(|c| *c.id()).collect();
    channel_ids.iter().for_each(|id| {
      self.db.as_mut().map(|db| {
        db.as_mut()
          .add_persist(ActionPersist::RemoveChannel { channel_id: *id })
      });
    });
    self.channels.clear();
  }
}

#[derive(Debug, Default)]
pub struct AppCfg {
  proxy: Option<String>,
  def_bot_id: Uuid,
}

impl AppCfg {
  pub fn new(proxy: Option<String>, def_bot_id: Uuid) -> Self { Self { proxy, def_bot_id } }

  #[inline]
  pub fn proxy(&self) -> Option<&str> { self.proxy.as_deref() }

  #[inline]
  pub fn set_proxy(&mut self, proxy: Option<String>) { self.proxy = proxy; }

  #[inline]
  pub fn def_bot_id(&self) -> &Uuid { &self.def_bot_id }
}
