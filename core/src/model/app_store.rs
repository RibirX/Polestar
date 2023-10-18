use super::{bot::Bot, channel::Channel, msg::Msg};
use derive_builder::Builder;
use uuid::Uuid;

#[derive(Debug, Builder, Default)]
#[builder(name = "AppBuilder")]
#[builder(pattern = "owned")]
#[builder(default)]
pub struct AppStore {
  bots: Vec<Bot>,
  msgs: Vec<Msg>,
  channels: Vec<Channel>,
  cur_channel_id: Uuid,
  cfg: AppCfg,
}

impl AppStore {
  pub fn bots(&self) -> &[Bot] { &self.bots }

  pub fn channels(&self) -> &[Channel] { &self.channels }

  pub fn msgs(&self) -> &[Msg] { &self.msgs }

  pub fn cfg(&self) -> &AppCfg { &self.cfg }

  pub fn switch_channel(&mut self, id: &Uuid) {
    let idx = self.channels.iter().position(|channel| channel.id() == id);

    if let Some(idx) = idx {
      self.cur_channel_id = *self.channels[idx].id();
    } else {
      log::warn!(
        "[polestar] switch channel error: channel id {} not found",
        id
      );
    }
  }

  pub fn add_channel(&mut self, channel: Channel) {
    let id = channel.id();

    if self.channels.iter().any(|channel| channel.id() == id) {
      log::warn!(
        "[polestar] add channel error: channel id {} already exists",
        id
      );
      return;
    }

    self.cur_channel_id = *id;
    self.channels.push(channel);
  }

  pub fn remove_channel(&mut self, id: &Uuid) {
    let idx = self.channels.iter().position(|channel| channel.id() == id);

    if let Some(idx) = idx {
      self.channels.remove(idx);

      if id == &self.cur_channel_id {
        if self.channels.is_empty() {
          self.cur_channel_id = Uuid::nil();
        } else if idx == 0 {
          self.cur_channel_id = *self.channels[idx].id();
        } else {
          self.cur_channel_id = *self.channels[idx - 1].id();
        }
      }
    } else {
      log::warn!(
        "[polestar] delete channel error: channel id {} not found",
        id
      );
    }
  }

  #[inline]
  pub fn add_msg(&mut self, msg: Msg) { self.msgs.push(msg); }

  #[inline]
  pub fn cur_channel_id(&self) -> &Uuid { &self.cur_channel_id }

  #[inline]
  pub fn cfg_as_ref(&self) -> &AppCfg { &self.cfg }

  #[inline]
  pub fn cfg_as_mut(&mut self) -> &mut AppCfg { &mut self.cfg }
}

#[derive(Debug, Default)]
pub struct AppCfg {
  proxy: Option<String>,
}

impl AppCfg {
  pub fn proxy(&self) -> Option<&str> { self.proxy.as_deref() }

  pub fn set_proxy(&mut self, proxy: Option<String>) { self.proxy = proxy; }
}

#[cfg(test)]
mod test {
  use crate::model::{
    channel::ChannelBuilder,
    msg::{MsgCont, MsgMeta, MsgRole},
  };

  use super::*;

  fn load_bots() -> Vec<Bot> {
    let preset_bot_cfg =
      serde_json::from_str::<serde_json::Value>(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/..", "/config/bot.json")))
        .expect("Failed to parse bot.json");
    let bots = serde_json::from_value::<Vec<Bot>>(preset_bot_cfg["bots"].clone())
      .expect("Failed to parse bots");
    bots
  }

  #[test]
  fn test_app_store() {
    let bots = load_bots();
    let mut app_store = AppBuilder::default().bots(bots).build().unwrap();
    assert_eq!(app_store.bots().len(), 1);

    app_store.add_msg(Msg::new(
      MsgRole::User,
      MsgCont::text_init(),
      MsgMeta::default(),
    ));
    assert_eq!(app_store.msgs().len(), 1);
  }

  #[test]
  fn test_app_chan() {
    testing_logger::setup();
    let mut app_store = AppBuilder::default().build().unwrap();
    assert_eq!(app_store.cur_channel_id(), &Uuid::nil());
    assert_eq!(app_store.channels().len(), 0);

    let channel_1 = ChannelBuilder::default().name("test 1").build().unwrap();
    let id_1 = *channel_1.id();
    app_store.add_channel(channel_1);

    assert_eq!(app_store.cur_channel_id(), &id_1);
    assert_eq!(app_store.channels().len(), 1);

    let channel_2 = ChannelBuilder::default().name("test 2").build().unwrap();
    let id_2 = *channel_2.id();
    app_store.add_channel(channel_2);

    assert_eq!(app_store.cur_channel_id(), &id_2);
    assert_eq!(app_store.channels().len(), 2);

    let channel_3 = ChannelBuilder::default().name("test 3").build().unwrap();
    let id_3 = *channel_3.id();
    app_store.add_channel(channel_3);

    assert_eq!(app_store.cur_channel_id(), &id_3);
    assert_eq!(app_store.channels().len(), 3);

    app_store.remove_channel(&id_1);
    assert_eq!(app_store.cur_channel_id(), &id_3);
    assert_eq!(app_store.channels().len(), 2);

    app_store.remove_channel(&id_3);
    assert_eq!(app_store.cur_channel_id(), &id_2);
    assert_eq!(app_store.channels().len(), 1);

    let channel_4 = ChannelBuilder::default().name("test 4").build().unwrap();
    let id_4 = *channel_4.id();
    app_store.add_channel(channel_4);

    assert_eq!(app_store.cur_channel_id(), &id_4);
    assert_eq!(app_store.channels().len(), 2);

    let channel_5 = ChannelBuilder::default().name("test 5").build().unwrap();
    let id_5 = *channel_5.id();
    app_store.add_channel(channel_5);

    assert_eq!(app_store.cur_channel_id(), &id_5);
    assert_eq!(app_store.channels().len(), 3);

    // switch non-exist channel, has log warning
    app_store.switch_channel(&id_1);

    app_store.switch_channel(&id_4);
    app_store.remove_channel(&id_4);
    assert_eq!(app_store.cur_channel_id(), &id_2);

    testing_logger::validate(|captured_logs| {
      assert_eq!(captured_logs.len(), 1);
      assert_eq!(
        captured_logs[0].body,
        format!(
          "[polestar] switch channel error: channel id {} not found",
          id_1
        ),
      );
    });
  }
}
