use once_cell::sync::Lazy;
use std::{collections::HashMap, ptr::NonNull, rc::Rc, sync::Mutex};
use uuid::Uuid;

use crate::{
  db::{executor::ActionPersist, pool::PersistenceDB},
  utils, LocalState,
};

use super::{
  bot::Bot,
  channel::{Channel, ChannelCfg},
  BotId, ChannelId, User, UserBuilder,
};

pub struct AppInfo {
  bots: Rc<Vec<Bot>>,
  user: Option<User>,
  cfg: AppCfg,
  cur_channel_id: Option<Uuid>,
  quick_launcher_id: Option<Uuid>,
  has_official_server: bool,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum GlbVar {
  Version,
  PolestarKey,
  UserAgent,
}

impl ToString for GlbVar {
  fn to_string(&self) -> String {
    match self {
      GlbVar::Version => "Version".to_owned(),
      GlbVar::PolestarKey => "PolestarKey".to_owned(),
      GlbVar::UserAgent => "UserAgent".to_owned(),
    }
  }
}

pub(crate) static GLOBAL_VARS: Lazy<Mutex<HashMap<GlbVar, String>>> = Lazy::new(|| {
  let mut vars = HashMap::new();
  vars.insert(GlbVar::Version, env!("CARGO_PKG_VERSION").to_owned());
  vars.insert(
    GlbVar::UserAgent,
    format!("PoleStarChat/{}", env!("CARGO_PKG_VERSION")),
  );
  Mutex::new(vars)
});

impl AppInfo {
  fn save_local(&self) {
    if let Some(user) = &self.user {
      let uid = user.uid();
      let local_state = LocalState::new(self.cur_channel_id, self.quick_launcher_id, Some(uid));
      utils::write_local_state(&uid.to_string(), &local_state).expect("Failed to save local state");
    }
  }

  pub fn user(&self) -> Option<&User> { self.user.as_ref() }

  pub fn set_user(&mut self, user: Option<User>) {
    self.user = user;
    self.save_local();
  }

  pub fn cur_channel_id(&self) -> Option<&Uuid> { self.cur_channel_id.as_ref() }

  pub fn set_cur_channel_id(&mut self, cur_channel_id: Option<Uuid>) {
    self.cur_channel_id = cur_channel_id;
    self.save_local();
  }

  pub fn quick_launcher_id(&self) -> Option<&Uuid> { self.quick_launcher_id.as_ref() }

  pub fn set_quick_launcher_id(&mut self, quick_launcher_id: Option<Uuid>) {
    self.quick_launcher_id = quick_launcher_id;
    self.save_local();
  }

  pub fn bots(&self) -> &[Bot] { &self.bots }

  pub fn bots_rc(&self) -> Rc<Vec<Bot>> { self.bots.clone() }

  pub fn bot(&self, bot_id: &BotId) -> Option<&Bot> {
    self.bots.iter().find(|bot| bot.id() == bot_id)
  }

  pub fn get_bot_or_default(&self, bot_id: Option<&BotId>) -> &Bot {
    bot_id
      .and_then(|bot_id| self.bot(bot_id))
      .unwrap_or_else(|| self.def_bot())
  }

  pub fn def_bot(&self) -> &Bot {
    self
      .bots
      .iter()
      .find(|bot| bot.id() == self.cfg.def_bot_id())
      .expect("default bot not found")
  }

  pub fn cfg(&self) -> &AppCfg { &self.cfg }

  pub fn cfg_mut(&mut self) -> &mut AppCfg { &mut self.cfg }

  pub fn has_official_server(&self) -> bool { self.has_official_server }

  pub fn need_login(&self) -> bool { self.has_official_server && self.user.is_none() }
}

pub struct AppData {
  channels: Vec<Channel>,
  db: Option<Box<PersistenceDB>>,
  info: Box<AppInfo>,
}

pub static ANONYMOUS_USER: &str = "anonymous";

pub fn init_app_data() -> AppData {
  utils::launch::setup_project();
  // 1. load bots config from local file.
  let bots = utils::load_bot_cfg_file().expect("Failed to load bot config");
  // 2. judge bot has official server.
  let has_official_server = utils::has_official_server(&bots);
  // TODO: how to set app default bot
  let cfg = AppCfg::new(None, bots[0].id().clone());
  // 3. if has official server, load user info from local file.
  let cur_user = utils::read_current_user().unwrap_or(ANONYMOUS_USER.to_owned());
  let local_state = utils::read_local_state(&cur_user).unwrap_or_default();
  let (user_data_path, user) = if has_official_server {
    local_state.uid().map_or_else(
      || (None, None),
      |uid| {
        let user_data_path = utils::user_data_path(&uid.to_string());
        let token = utils::token::decrypt_token(crate::KEY).ok();
        let mut user_builder = UserBuilder::default();
        user_builder = user_builder.uid(uid);
        if let Some(token) = token {
          GLOBAL_VARS
            .try_lock()
            .unwrap()
            .insert(GlbVar::PolestarKey, token.to_owned());
          user_builder = user_builder.token(token);
        }
        (
          Some(user_data_path),
          Some(user_builder.build().expect("Failed to build user")),
        )
      },
    )
  } else {
    let anonymous_data_path = utils::user_data_path(ANONYMOUS_USER);
    (Some(anonymous_data_path), None)
  };

  let (db, mut channels) = if let Some(user_data_path) = user_data_path {
    utils::create_if_not_exist_dir(user_data_path);
    init_db(user.as_ref().map(|user| user.uid()))
  } else {
    (None, vec![])
  };

  let cur_channel_id = local_state.cur_channel_id();

  let cur_channel = channels
    .iter()
    .find(|channel| Some(*channel.id()) == cur_channel_id.map(|id| id));

  let cur_channel_id = if cur_channel.is_some() {
    *cur_channel_id
  } else {
    // channels reverse display
    channels.last().map(|channel| *channel.id())
  };

  let info = AppInfo {
    bots: Rc::new(bots),
    user,
    cfg,
    cur_channel_id,
    quick_launcher_id: *local_state.quick_launcher_id(),
    has_official_server,
  };

  let info = Box::new(info);
  let ptr = NonNull::from(&*info);

  channels.iter_mut().for_each(|channel| {
    channel.set_app_info(ptr);
  });

  AppData::new(channels, db, info)
}

#[cfg(feature = "persistence")]
fn init_db(uid: Option<u64>) -> (Option<Box<PersistenceDB>>, Vec<Channel>) {
  use crate::db::pool::{db_path, init_db, runtime};
  let db = PersistenceDB::connect(init_db(&db_path(uid))).expect("Failed to connect db");

  let mut channels =
    runtime().block_on(async { db.query_channels().await.expect("Failed to query channels") });

  channels.iter_mut().for_each(|channel| {
    let msgs = runtime().block_on(async {
      db.query_msgs_by_channel_id(channel.id())
        .await
        .expect("Failed to query msgs")
    });
    channel.load_msgs(msgs);
  });

  let db = Box::new(db);

  // if channels is empty, create a default channel.
  if channels.is_empty() {
    let channel_id = Uuid::new_v4();
    let channel = Channel::new(
      channel_id,
      "Untitled".to_owned(),
      None,
      ChannelCfg::default(),
      None,
      None,
    );
    db.persist_async(ActionPersist::AddChannel {
      id: channel_id,
      name: "Untitled".to_owned(),
      desc: None,
      cfg: ChannelCfg::default(),
    });
    channels.push(channel);
  }

  let ptr = NonNull::from(&*db);
  channels.iter_mut().for_each(|channel| {
    channel.set_db(ptr);
  });
  (Some(db), channels)
}

#[cfg(not(feature = "persistence"))]
fn init_db(_uid: Option<u64>) -> (Option<Box<PersistenceDB>>, Vec<Channel>) {
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
    channels: Vec<Channel>,
    db: Option<Box<PersistenceDB>>,
    info: Box<AppInfo>,
  ) -> Self {
    Self { channels, db, info }
  }

  #[inline]
  pub fn info(&self) -> &AppInfo { &self.info }

  pub fn info_mut(&mut self) -> &mut AppInfo { &mut self.info }

  #[inline]
  pub fn channels(&self) -> &[Channel] { &self.channels }

  #[inline]
  pub fn channels_mut(&mut self) -> &mut Vec<Channel> { &mut self.channels }

  pub fn get_channel(&self, channel_id: &Uuid) -> Option<&Channel> {
    self
      .channels
      .iter()
      .find(|channel| channel.id() == channel_id)
  }

  pub fn get_channel_mut(&mut self, channel_id: &ChannelId) -> Option<&mut Channel> {
    self
      .channels
      .iter_mut()
      .find(|channel| channel.id() == channel_id)
  }

  pub fn switch_channel(&mut self, channel_id: &ChannelId) {
    self.info.as_mut().set_cur_channel_id(Some(*channel_id));
  }

  pub fn cur_channel(&self) -> Option<&Channel> {
    self
      .channels
      .iter()
      .find(|channel| Some(channel.id()) == self.info.cur_channel_id.as_ref())
  }

  pub fn cur_channel_mut(&mut self) -> Option<&mut Channel> {
    self
      .channels
      .iter_mut()
      .find(|channel| Some(channel.id()) == self.info.cur_channel_id.as_ref())
  }

  pub fn new_channel(&mut self, name: String, desc: Option<String>, cfg: ChannelCfg) -> Uuid {
    let channel_id = Uuid::new_v4();
    let db = self.db.as_mut().map(|db| NonNull::from(&**db));
    let info = &mut self.info;
    let app_info = Some(NonNull::from(&**info));
    let channel = Channel::new(channel_id, name, desc, cfg, app_info, db);

    let p_channel = channel.clone();

    if let Some(db) = self.db.as_mut() {
      db.persist_async(ActionPersist::AddChannel {
        id: channel_id,
        name: p_channel.name().to_owned(),
        desc: p_channel.desc().map(String::from),
        cfg: p_channel.cfg().clone(),
      });
    }

    self.channels.push(channel);
    self.info.as_mut().set_cur_channel_id(Some(channel_id));

    channel_id
  }

  pub fn remove_channel(&mut self, channel_id: &Uuid) {
    // guard channels is empty after remove channel
    if self.channels.len() == 1 {
      self.new_channel("Untitled".to_owned(), None, ChannelCfg::default());
    }
    if let Some(db) = self.db.as_mut() {
      db.persist_async(ActionPersist::RemoveChannel { channel_id: *channel_id })
    }

    // if current channel is removed, switch to nearest channel.
    if Some(*channel_id) == self.info.cur_channel_id {
      let cur_channel_id = self
        .channels
        .iter()
        .position(|channel| Some(channel.id()) == self.info.cur_channel_id.as_ref())
        .map(|idx| {
          if idx == 0 {
            *self.channels[1].id()
          } else {
            *self.channels[idx - 1].id()
          }
        });
      self.info.as_mut().set_cur_channel_id(cur_channel_id);
    }

    self.channels.retain(|channel| channel.id() != channel_id);
  }

  pub fn login(&mut self, user: User) {
    let uid = user.uid();
    self.info.as_mut().set_user(Some(user));
    let local_state = utils::read_local_state(&uid.to_string()).unwrap_or_default();
    self
      .info
      .as_mut()
      .set_quick_launcher_id(*local_state.quick_launcher_id());

    let (db, mut channels) = init_db(Some(uid));
    self.db = db;

    let cur_channel_id = local_state.cur_channel_id();

    let cur_channel = channels
      .iter()
      .find(|channel| Some(*channel.id()) == cur_channel_id.map(|id| id));

    let cur_channel_id = if cur_channel.is_some() {
      *cur_channel_id
    } else {
      // channels reverse display
      channels.last().map(|channel| *channel.id())
    };

    self.info.as_mut().set_cur_channel_id(cur_channel_id);

    let ptr = NonNull::from(&*self.info);
    channels.iter_mut().for_each(|channel| {
      channel.set_app_info(ptr);
    });

    self.channels = channels;
  }

  pub fn logout(&mut self) {
    self.info.as_mut().set_user(None);
    self.info.as_mut().set_quick_launcher_id(None);
    self.db = None;
    utils::del_current_user().expect("Failed to delete current user");
    utils::token::del_token().expect("Failed to delete token");
  }
}

#[derive(Debug, Default)]
pub struct AppCfg {
  proxy: Option<String>,
  def_bot_id: String,
}

impl AppCfg {
  pub fn new(proxy: Option<String>, def_bot_id: BotId) -> Self { Self { proxy, def_bot_id } }

  #[inline]
  pub fn proxy(&self) -> Option<&str> { self.proxy.as_deref() }

  #[inline]
  pub fn set_proxy(&mut self, proxy: Option<String>) { self.proxy = proxy; }

  #[inline]
  pub fn def_bot_id(&self) -> &BotId { &self.def_bot_id }
}
