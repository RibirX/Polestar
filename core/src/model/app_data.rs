use std::{pin::Pin, ptr::NonNull, rc::Rc};

use uuid::Uuid;

use crate::{
  db::{executor::ActionPersist, pool::PersistenceDB},
  utils,
};

use super::{
  bot::Bot,
  channel::{Channel, ChannelCfg},
  User,
};

pub struct AppData {
  bots: Rc<Vec<Bot>>,
  channels: Vec<Channel>,
  cur_channel_id: Uuid,
  cfg: AppCfg,
  user: Option<User>,
  db: Option<Pin<Box<PersistenceDB>>>,
}

#[cfg(not(feature = "persistence"))]
pub fn init_app_data() -> AppData {
  utils::launch::setup_project();
  // 1. load bots config from local file.
  // TODO: need handle load bot error.
  let bots = utils::load_bot_cfg_file().expect("Failed to load bot config");
  // 2. judge bot has official server.
  let has_official_server = utils::has_official_server(&bots);
  // 3. if has official server, load user info from local file.
  // TODO: load user info

  let cfg = AppCfg::new(None, bots[0].id().clone(), has_official_server);
  let channels = serde_json::from_str::<Vec<Channel>>(include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/..",
    "/gui/channels_mock.json"
  )))
  .expect("Failed to load mock data");
  let cur_channel_id = channels[0].id().clone();
  AppData::new(bots, channels, cur_channel_id, None, cfg)
}

#[cfg(feature = "persistence")]
pub fn init_app_data() -> AppData {
  use crate::db::pool::{db_path, init_db};
  use std::time::Duration;

  utils::launch::setup_project();
  // 1. load bots config from local file.
  let bots = utils::load_bot_cfg_file().expect("Failed to load bot config");
  // 2. judge bot has official server.
  let has_official_server = utils::has_official_server(&bots);
  // 3. if has official server, load user info from local file.
  // TODO: load user info

  let cfg = AppCfg::new(None, bots[0].id().clone(), has_official_server);

  let db = PersistenceDB::connect(init_db(&db_path()), Duration::from_secs(1))
    .expect("Failed to connect db");
  let channels = db.query_channels().expect("Failed to query channels");

  let cur_channel_id = channels[0].id().clone();
  AppData::new(bots, channels, cur_channel_id, Some(Box::pin(db)), cfg)
}

impl AppData {
  pub fn new(
    bots: Vec<Bot>,
    channels: Vec<Channel>,
    cur_channel_id: Uuid,
    db: Option<Pin<Box<PersistenceDB>>>,
    cfg: AppCfg,
  ) -> Self {
    Self {
      bots: Rc::new(bots),
      channels,
      cur_channel_id,
      cfg,
      user: None,
      db,
    }
  }

  #[inline]
  pub fn cur_channel_id(&self) -> &Uuid { &self.cur_channel_id }

  #[inline]
  pub fn bots(&self) -> &Vec<Bot> { &self.bots }

  #[inline]
  pub fn bots_rc(&self) -> Rc<Vec<Bot>> { self.bots.clone() }

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

  pub fn def_bot(&self) -> &Bot {
    self
      .bots
      .iter()
      .find(|bot| bot.id() == self.cfg.def_bot_id())
      .expect("default bot not found")
  }
}

#[derive(Debug, Default)]
pub struct AppCfg {
  proxy: Option<String>,
  def_bot_id: Uuid,
  has_official_server: bool,
}

impl AppCfg {
  pub fn new(proxy: Option<String>, def_bot_id: Uuid, has_official_server: bool) -> Self {
    Self {
      proxy,
      def_bot_id,
      has_official_server,
    }
  }

  #[inline]
  pub fn proxy(&self) -> Option<&str> { self.proxy.as_deref() }

  #[inline]
  pub fn set_proxy(&mut self, proxy: Option<String>) { self.proxy = proxy; }

  #[inline]
  pub fn def_bot_id(&self) -> &Uuid { &self.def_bot_id }
}
