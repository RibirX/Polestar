use polestar_core::model::{
  init_app_data, AppData, AppInfo, Bot, BotId, Channel, ChannelCfg, ChannelId, Msg, MsgAction,
  MsgCont, MsgId, User,
};
use ribir::prelude::*;
use ribir_algo::Sc;
use std::{collections::HashMap, rc::Rc};
use url::Url;
use uuid::Uuid;

use super::{
  common::{PartialPath, Route, Router, Tooltip},
  home::w_home,
  login::w_login,
  permission::w_permission,
};
use crate::{theme::polestar_theme, widgets::modify_channel::w_modify_channel_modal, WINDOW_MGR};

pub trait Chat: 'static {
  fn add_msg(&mut self, channel_id: &ChannelId, msg: Msg);
  fn add_msg_cont(
    &mut self,
    channel_id: &ChannelId,
    msg_id: &MsgId,
    cont: MsgCont,
  ) -> Option<usize>;

  fn switch_cont(&mut self, channel_id: &ChannelId, msg_id: &MsgId, idx: usize);

  fn update_msg_cont(&mut self, channel_id: &ChannelId, msg_id: &MsgId, idx: usize, act: MsgAction);

  fn channel(&self, channel_id: &ChannelId) -> Option<&Channel>;

  fn msg(&self, channel_id: &ChannelId, msg_id: &MsgId) -> Option<&Msg>;

  fn info(&self) -> &AppInfo;
}

pub trait ChannelMgr: 'static {
  fn cur_channel_id(&self) -> Option<&ChannelId>;
  fn channel_ids(&self) -> Vec<ChannelId>;
  fn channel(&self, channel_id: &ChannelId) -> Option<&Channel>;
  fn switch_channel(&mut self, channel_id: &ChannelId);
  fn new_channel(&mut self, name: String, desc: Option<String>, cfg: ChannelCfg) -> Uuid;
  fn update_channel_desc(&mut self, channel_id: &ChannelId, desc: Option<String>);
  fn update_channel_name(&mut self, channel_id: &ChannelId, name: String);
  fn update_channel_cfg(&mut self, channel_id: &ChannelId, cfg: ChannelCfg);
  fn remove_channel(&mut self, channel_id: &ChannelId);
}

pub trait UIState: 'static {
  fn cur_path(&self) -> String;
  fn navigate_to(&mut self, path: &str);
  fn modify_channel_id(&self) -> Option<&Uuid>;
  fn set_modify_channel_id(&mut self, modify_channel_id: Option<Uuid>);
  fn tooltip(&self) -> Option<String>;
  fn set_tooltip(&mut self, tooltip: Option<&str>);
}

pub trait UserConfig: 'static {
  fn login(&mut self, user: User) -> ChannelId;
  fn user(&self) -> Option<&User>;
  fn logout(&mut self);
  fn need_login(&self) -> bool;
  fn bots(&self) -> Rc<Vec<Bot>>;
  fn default_bot_id(&self) -> BotId;
}

pub struct AppGUI {
  data: AppData,
  cur_router_path: String,
  modify_channel_id: Option<Uuid>,
  tooltip: Option<String>,
}

impl AppGUI {
  fn new(data: AppData) -> Self {
    let cur_router_path = if data.info().need_login() {
      "/login".to_owned()
    } else {
      "/home/chat".to_owned()
    };

    Self {
      data,
      // It can get by open app url, like this: PoleStarChat://ribir.org/home/chat
      cur_router_path,
      modify_channel_id: None,
      tooltip: None,
    }
  }

  fn chat(&self) -> &dyn Chat { self }
  fn chat_mut(&mut self) -> &mut dyn Chat { self }

  fn channel_mgr(&self) -> &dyn ChannelMgr { self }
  fn channel_mgr_mut(&mut self) -> &mut dyn ChannelMgr { self }

  fn ui_state(&self) -> &dyn UIState { self }
  fn ui_state_mut(&mut self) -> &mut dyn UIState { self }

  fn user_config(&self) -> &dyn UserConfig { self }
  fn user_config_mut(&mut self) -> &mut dyn UserConfig { self }
}

impl ChannelMgr for AppGUI {
  fn channel_ids(&self) -> Vec<ChannelId> {
    self
      .data
      .channels()
      .iter()
      .map(|channel| channel.id())
      .cloned()
      .collect()
  }

  fn cur_channel_id(&self) -> Option<&ChannelId> { self.data.info().cur_channel_id() }

  fn channel(&self, channel_id: &ChannelId) -> Option<&Channel> {
    self.data.get_channel(channel_id)
  }

  fn new_channel(&mut self, name: String, desc: Option<String>, cfg: ChannelCfg) -> Uuid {
    self.data.new_channel(name, desc, cfg)
  }

  fn remove_channel(&mut self, channel_id: &ChannelId) { self.data.remove_channel(channel_id); }

  fn switch_channel(&mut self, channel_id: &ChannelId) { self.data.switch_channel(channel_id); }

  fn update_channel_desc(&mut self, channel_id: &ChannelId, desc: Option<String>) {
    if let Some(channel) = self.data.get_channel_mut(channel_id) {
      channel.set_desc(desc);
    }
  }

  fn update_channel_name(&mut self, channel_id: &ChannelId, name: String) {
    if let Some(channel) = self.data.get_channel_mut(channel_id) {
      channel.set_name(name);
    }
  }

  fn update_channel_cfg(&mut self, channel_id: &ChannelId, cfg: ChannelCfg) {
    if let Some(channel) = self.data.get_channel_mut(channel_id) {
      channel.set_cfg(cfg);
    }
  }
}

impl Chat for AppGUI {
  fn add_msg(&mut self, channel_id: &ChannelId, msg: Msg) {
    if let Some(ch) = self.data.get_channel_mut(channel_id) {
      ch.add_msg(msg);
    }
  }

  fn add_msg_cont(
    &mut self,
    channel_id: &ChannelId,
    msg_id: &MsgId,
    cont: MsgCont,
  ) -> Option<usize> {
    self
      .data
      .get_channel_mut(channel_id)
      .and_then(|ch| ch.msg_mut(msg_id))
      .map(|msg| msg.add_cont(cont))
  }

  fn switch_cont(&mut self, channel_id: &ChannelId, msg_id: &MsgId, idx: usize) {
    if let Some(msg) = self
      .data
      .get_channel_mut(channel_id)
      .and_then(|ch| ch.msg_mut(msg_id))
    {
      msg.switch_cont(idx);
    }
  }

  fn update_msg_cont(
    &mut self,
    channel_id: &ChannelId,
    msg_id: &MsgId,
    idx: usize,
    act: MsgAction,
  ) {
    if let Some(ch) = self.data.get_channel_mut(channel_id) {
      ch.update_msg(msg_id, idx, act);
    }
  }

  fn channel(&self, channel_id: &ChannelId) -> Option<&Channel> {
    self.data.get_channel(channel_id)
  }

  fn info(&self) -> &AppInfo { self.data.info() }

  fn msg(&self, channel_id: &ChannelId, msg_id: &MsgId) -> Option<&Msg> {
    self
      .data
      .get_channel(channel_id)
      .and_then(|ch| ch.msg(msg_id))
  }
}

impl UIState for AppGUI {
  fn cur_path(&self) -> String { self.cur_router_path.clone() }

  fn navigate_to(&mut self, path: &str) { self.cur_router_path = path.to_owned(); }

  fn modify_channel_id(&self) -> Option<&Uuid> { self.modify_channel_id.as_ref() }

  fn set_modify_channel_id(&mut self, modify_channel_id: Option<Uuid>) {
    self.modify_channel_id = modify_channel_id;
  }

  fn set_tooltip(&mut self, tooltip: Option<&str>) { self.tooltip = tooltip.map(|s| s.to_owned()); }

  fn tooltip(&self) -> Option<String> { self.tooltip.clone() }
}

impl UserConfig for AppGUI {
  fn login(&mut self, user: User) -> ChannelId {
    self.data.login(user);
    self.data.info().cur_channel_id().cloned().unwrap()
  }

  fn logout(&mut self) { self.data.logout(); }

  fn need_login(&self) -> bool { self.data.info().need_login() }

  fn user(&self) -> Option<&User> { self.data.info().user() }

  fn bots(&self) -> Rc<Vec<Bot>> { self.data.info().bots_rc() }

  fn default_bot_id(&self) -> BotId { self.data.info().cfg().def_bot_id().clone() }
}

impl Compose for AppGUI {
  fn compose(this: impl StateWriter<Value = Self>) -> impl WidgetBuilder {
    fn_widget! {
      let chat = this.split_writer(|app| app.chat(), |app| app.chat_mut());
      let channel_mgr = this.split_writer(|app| app.channel_mgr(), |app| app.channel_mgr_mut());
      let ui_state = this.split_writer(|app| app.ui_state(), |app| app.ui_state_mut());
      let config = this.split_writer(|app| app.user_config(), |app| app.user_config_mut());
      App::events_stream()
        .subscribe(
          gen_handler(config.clone_writer(), channel_mgr.clone_writer(), ui_state.clone_writer())
        );

      @ThemeWidget {
        // Polestar custom theme.
        theme: Sc::new(Theme::Inherit(polestar_theme())),
        @ {
          Box::new(fn_widget! {
            @Stack {
              @Router {
                cur_path: pipe!($ui_state.cur_path().to_owned()),
                @Route {
                  path: PartialPath::new("/login", 0),
                  @ { w_login(config.clone_writer()) }
                }
                @Route {
                  path: PartialPath::new("/permission", 0),
                  @ { w_permission() }
                }
                @Route {
                  path: PartialPath::new("/home", 0),
                  @ {
                    w_home(
                      chat.clone_writer(),
                      channel_mgr.clone_writer(),
                      config.clone_writer(),
                      ui_state.clone_writer()
                    )
                  }
                }
              }
              @ {
                pipe! {
                  w_tooltip($this.tooltip())
                }
              }
              @ {
                pipe!($ui_state;)
                  .map(move |_| {
                    let _ = || {
                      $channel_mgr.write();
                      $ui_state.write();
                      $config.write();
                    };
                    let modify_channel_id = $ui_state.modify_channel_id().cloned();
                    modify_channel_id.map(|modify_channel_id| {
                      w_modify_channel_modal(
                        channel_mgr.clone_writer(),
                        ui_state.clone_writer(),
                        config.clone_writer(),
                        &modify_channel_id
                      )
                    })
                  })
              }
            }
          })
        }
      }
    }
  }
}

pub enum AppRoute {
  Login { token: String, uid: u64 },
}

fn handle_open_url(url: &str) -> Option<AppRoute> {
  let url_parser = Url::parse(url).expect("");
  let path_segments = url_parser.path_segments();
  let mut segments = vec![];
  if let Some(mut path_segments) = path_segments {
    let mut segment = path_segments.next();
    while let Some(s) = segment {
      segments.push(s);
      segment = path_segments.next();
    }
  }

  let mut query_pairs = url_parser.query_pairs();
  let mut pair = query_pairs.next();
  let mut params = HashMap::new();

  while let Some(p) = pair {
    params.insert(p.0, p.1);
    pair = query_pairs.next();
  }

  if segments.len() == 1 {
    if let Some(segment) = segments.first() {
      if *segment == "login" {
        if let Some(token) = params.get("token") {
          if let Some(uid) = params.get("user_id") {
            return Some(AppRoute::Login {
              token: token.to_string(),
              uid: uid.parse::<u64>().unwrap(),
            });
          }
        }
      }
    }
  }

  None
}

fn gen_handler(
  config: impl StateWriter<Value = dyn UserConfig>,
  channel_mgr: impl StateWriter<Value = dyn ChannelMgr>,
  ui_state: impl StateWriter<Value = dyn UIState>,
) -> impl for<'a> FnMut(&'a mut AppEvent) {
  move |event: &mut AppEvent| {
    if let AppEvent::OpenUrl(url) = event {
      // TODO: user module need login
      let route = handle_open_url(url);
      if let Some(AppRoute::Login { token, uid }) = route {
        let _ = polestar_core::token::encrypt_token(token.as_bytes());

        // create uid user folder.
        let user_data_path = polestar_core::user_data_path(&uid.to_string());
        polestar_core::create_if_not_exist_dir(user_data_path);

        let _ = polestar_core::write_current_user(&uid.to_string());

        // create `User`
        let mut user = polestar_core::model::UserBuilder::default()
          .uid(uid)
          .build()
          .expect("Failed to build user");
        user.set_token(Some(token));

        let channel_id = config.write().login(user);
        channel_mgr.write().switch_channel(&channel_id);
        ui_state.write().navigate_to("/home/chat");

        // active main window
        if let Some(wnd_info) = WINDOW_MGR.lock().unwrap().main.as_ref() {
          App::set_active_window(wnd_info.id);
        }
      }
    }
  }
}

fn w_tooltip(content: Option<String>) -> Option<impl WidgetBuilder> {
  content.map(|content| {
    fn_widget! {
      @Tooltip { content }
    }
  })
}

// launch App need to do some init work.
// 1. [x] load bot config file.
// 2. [ ] if has user module, need check user login status.
// 3. [ ] some static file can't be load dynamic, need load as bytes before
//    launch.
// 4. [ ] load local db data.
pub fn w_app() -> impl WidgetBuilder {
  let app_data = init_app_data();
  fn_widget! { AppGUI::new(app_data) }
}
