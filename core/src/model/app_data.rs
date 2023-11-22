use std::{pin::Pin, ptr::NonNull, rc::Rc};

use uuid::Uuid;

use crate::{
  db::{executor::ActionPersist, pool::PersistenceDB},
  utils::{self, LocalState},
};

use super::{
  bot::Bot,
  channel::{Channel, ChannelCfg},
  User, UserBuilder,
};

pub struct AppData {
  bots: Rc<Vec<Bot>>,
  channels: Vec<Channel>,
  cur_channel_id: Option<Uuid>,
  cfg: AppCfg,
  user: Option<User>,
  db: Option<Pin<Box<PersistenceDB>>>,
  local_state: LocalState,
}

pub fn init_app_data() -> AppData {
  utils::launch::setup_project();
  // 1. load bots config from local file.
  // TODO: need handle load bot error.
  let bots = utils::load_bot_cfg_file().expect("Failed to load bot config");
  // 2. judge bot has official server.
  let has_official_server = utils::has_official_server(&bots);
  // 3. if has official server, load user info from local file.
  // TODO: load user info from local file.
  let cur_user = utils::read_current_user().unwrap_or_default();
  let mut local_state = utils::read_local_state(&cur_user).unwrap_or_default();
  let (user_data_path, user) = if has_official_server {
    local_state.uid().map_or_else(
      || {
        let anonymous_data_path = utils::user_data_path("anonymous");
        (anonymous_data_path, None)
      },
      |uid| {
        let user_data_path = utils::user_data_path(&uid.to_string());
        // TODO: get token from local file.
        let token = utils::token::decrypt_token(crate::KEY).ok();
        println!("token: {:?}", token);
        let mut user_builder = UserBuilder::default();
        user_builder = user_builder.uid(uid);
        if let Some(token) = token {
          user_builder = user_builder.token(token);
        }
        (
          user_data_path,
          Some(user_builder.build().expect("Failed to build user")),
        )
      },
    )
  } else {
    let anonymous_data_path = utils::user_data_path("anonymous");
    (anonymous_data_path, None)
  };

  utils::create_if_not_exist_dir(user_data_path);

  // TODO: how to set app default bot
  let cfg = AppCfg::new(None, bots[0].id().clone(), has_official_server);
  let (db, channels) = init_db();

  let cur_channel_id = local_state.cur_channel_id();

  let cur_channel = channels
    .iter()
    .find(|channel| Some(*channel.id()) == cur_channel_id.map(|id| id));

  let cur_channel_id = if cur_channel.is_some() {
    *cur_channel_id
  } else {
    let cur_channel_id = channels.first().map(|channel| *channel.id());
    local_state.set_cur_channel_id(cur_channel_id);
    cur_channel_id
  };

  AppData::new(bots, channels, cur_channel_id, cfg, user, db, local_state)
}

#[cfg(feature = "persistence")]
fn init_db() -> (Option<Pin<Box<PersistenceDB>>>, Vec<Channel>) {
  use crate::db::pool::{db_path, init_db};
  use std::time::Duration;

  let db = PersistenceDB::connect(init_db(&db_path()), Duration::from_secs(1))
    .expect("Failed to connect db");
  let channels = db.query_channels().expect("Failed to query channels");
  (Some(Box::pin(db)), channels)
}

#[cfg(not(feature = "persistence"))]
fn init_db() -> (Option<Pin<Box<PersistenceDB>>>, Vec<Channel>) {
  use crate::db::executor::channel;

  let channels = serde_json::from_str::<Vec<Channel>>(include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/..",
    "/gui/channels_mock.json"
  )))
  .expect("Failed to load mock data");
  (None, channels)
}

impl AppData {
  pub(crate) fn new(
    bots: Vec<Bot>,
    channels: Vec<Channel>,
    cur_channel_id: Option<Uuid>,
    cfg: AppCfg,
    user: Option<User>,
    db: Option<Pin<Box<PersistenceDB>>>,
    local_state: LocalState,
  ) -> Self {
    Self {
      bots: Rc::new(bots),
      channels,
      cur_channel_id,
      cfg,
      user,
      db,
      local_state,
    }
  }

  #[inline]
  pub fn user(&self) -> Option<&User> { self.user.as_ref() }

  #[inline]
  pub fn set_user(&mut self, user: Option<User>) { self.user = user; }

  #[inline]
  pub fn cur_channel_id(&self) -> Option<&Uuid> { self.cur_channel_id.as_ref() }

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

  pub fn switch_channel(&mut self, channel_id: &Uuid) { self.cur_channel_id = Some(*channel_id); }

  pub fn cur_channel(&self) -> Option<&Channel> {
    self
      .channels
      .iter()
      .find(|channel| Some(channel.id()) == self.cur_channel_id.as_ref())
  }

  pub fn cur_channel_mut(&mut self) -> Option<&mut Channel> {
    self
      .channels
      .iter_mut()
      .find(|channel| Some(channel.id()) == self.cur_channel_id.as_ref())
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
    self.cur_channel_id = Some(id);

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

  pub fn local_state(&self) -> &LocalState { &self.local_state }

  pub fn local_state_mut(&mut self) -> &mut LocalState { &mut self.local_state }

  pub fn need_login(&self) -> bool { self.cfg.has_official_server && self.user.is_none() }
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

  #[inline]
  pub fn has_official_server(&self) -> bool { self.has_official_server }
}
