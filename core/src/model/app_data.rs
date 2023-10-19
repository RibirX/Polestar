use crate::chat_data::{ChatData, ChatDataOperator};

use super::bot::Bot;
use derive_builder::Builder;

#[derive(Builder, Default)]
#[builder(name = "AppBuilder")]
#[builder(pattern = "owned")]
#[builder(default)]
pub struct AppData<T: ChatDataOperator> {
  bots: Vec<Bot>,
  chat_data: ChatData<T>,
  cfg: AppCfg,
}

impl<T: ChatDataOperator> AppData<T> {
  #[inline]
  pub fn bots(&self) -> &[Bot] { &self.bots }

  #[inline]
  pub fn chat_data_ref(&self) -> &ChatData<T> { &self.chat_data }

  #[inline]
  pub fn chat_data_mut(&mut self) -> &mut ChatData<T> { &mut self.chat_data }

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
  use uuid::Uuid;

  use crate::model::{
    attachment::MIME,
    channel::ChannelBuilder,
    msg::{Msg, MsgCont, MsgMeta, MsgRole},
  };

  use super::*;

  #[derive(Default)]
  struct MockChatDataStore;

  impl ChatDataOperator for MockChatDataStore {
    fn add_msg(&self, _channel_id: &Uuid, _msg: &Msg) -> Result<(), crate::error::PolestarError> {
      Ok(())
    }

    fn query_msg_by_id(&self, _id: &Uuid) -> Result<Msg, crate::error::PolestarError> {
      Ok(Msg::new(
        MsgRole::User,
        MsgCont::text_init(),
        MsgMeta::default(),
      ))
    }

    fn update_msg(&self, _msg: &Msg) -> Result<(), crate::error::PolestarError> { Ok(()) }

    fn query_msgs_by_channel_id(
      &self,
      _id: &Uuid,
    ) -> Result<Vec<Msg>, crate::error::PolestarError> {
      Ok(vec![Msg::new(
        MsgRole::User,
        MsgCont::text_init(),
        MsgMeta::default(),
      )])
    }

    fn query_latest_text_msgs_by_channel_id(
      &self,
      _id: &Uuid,
      _limit: u32,
    ) -> Result<Vec<Msg>, crate::error::PolestarError> {
      Ok(vec![Msg::new(
        MsgRole::User,
        MsgCont::text_init(),
        MsgMeta::default(),
      )])
    }

    fn add_channel(
      &self,
      _channel: &crate::model::channel::Channel,
    ) -> Result<(), crate::error::PolestarError> {
      Ok(())
    }

    fn remove_channel(&self, _id: &Uuid) -> Result<(), crate::error::PolestarError> { Ok(()) }

    fn query_channel_by_id(
      &self,
      _id: &Uuid,
    ) -> Result<crate::model::channel::Channel, crate::error::PolestarError> {
      Ok(ChannelBuilder::default().name("test").build().unwrap())
    }

    fn update_channel(
      &self,
      _channel: &crate::model::channel::Channel,
    ) -> Result<(), crate::error::PolestarError> {
      Ok(())
    }

    fn query_channels(
      &self,
    ) -> Result<Vec<crate::model::channel::Channel>, crate::error::PolestarError> {
      Ok(vec![
        ChannelBuilder::default().name("test").build().unwrap(),
      ])
    }

    fn add_attachment(
      &self,
      _attachment: &crate::model::attachment::Attachment,
    ) -> Result<Uuid, crate::error::PolestarError> {
      Ok(Uuid::nil())
    }

    fn query_attachment_by_name(
      &self,
      _name: &Uuid,
    ) -> Result<crate::model::attachment::Attachment, crate::error::PolestarError> {
      Ok(crate::model::attachment::Attachment::new(
        MIME::ImagePng,
        [0, 1, 2, 3].to_vec(),
      ))
    }
  }

  fn load_bots() -> Vec<Bot> {
    let preset_bot_cfg = serde_json::from_str::<serde_json::Value>(include_str!(concat!(
      env!("CARGO_MANIFEST_DIR"),
      "/..",
      "/config/bot.json"
    )))
    .expect("Failed to parse bot.json");

    serde_json::from_value::<Vec<Bot>>(preset_bot_cfg["bots"].clone())
      .expect("Failed to parse bots")
  }

  #[test]
  fn test_app_data() {
    let bots = load_bots();
    let chat_data = ChatData::<MockChatDataStore>::default();
    let mut app_data = AppBuilder::default()
      .bots(bots)
      .chat_data(chat_data)
      .build()
      .unwrap();
    assert_eq!(app_data.bots().len(), 1);

    let add_msg_rst = app_data.chat_data_mut().add_msg(Msg::new(
      MsgRole::User,
      MsgCont::text_init(),
      MsgMeta::default(),
    ));
    assert!(add_msg_rst.is_ok());
    assert_eq!(app_data.chat_data_ref().msgs().len(), 1);
  }

  #[test]
  fn test_app_channel() {
    testing_logger::setup();
    let chat_data = ChatData::<MockChatDataStore>::default();
    let mut app_data = AppBuilder::default().chat_data(chat_data).build().unwrap();
    assert_eq!(app_data.chat_data_ref().cur_channel_id(), &Uuid::nil());
    assert_eq!(app_data.chat_data_ref().channels().len(), 0);

    let channel_1 = ChannelBuilder::default().name("test 1").build().unwrap();
    let id_1 = *channel_1.id();
    let add_channel_rst = app_data.chat_data_mut().add_channel(channel_1);

    assert!(add_channel_rst.is_ok());

    assert_eq!(app_data.chat_data_ref().cur_channel_id(), &id_1);
    assert_eq!(app_data.chat_data_ref().channels().len(), 1);

    let channel_2 = ChannelBuilder::default().name("test 2").build().unwrap();
    let id_2 = *channel_2.id();
    let add_channel_rst = app_data.chat_data_mut().add_channel(channel_2);
    assert!(add_channel_rst.is_ok());

    assert_eq!(app_data.chat_data_ref().cur_channel_id(), &id_2);
    assert_eq!(app_data.chat_data_ref().channels().len(), 2);

    let channel_3 = ChannelBuilder::default().name("test 3").build().unwrap();
    let id_3 = *channel_3.id();
    let add_channel_rst = app_data.chat_data_mut().add_channel(channel_3);
    assert!(add_channel_rst.is_ok());

    assert_eq!(app_data.chat_data_ref().cur_channel_id(), &id_3);
    assert_eq!(app_data.chat_data_ref().channels().len(), 3);

    let remove_channel_rst = app_data.chat_data_mut().remove_channel(&id_1);
    assert!(remove_channel_rst.is_ok());

    assert_eq!(app_data.chat_data_ref().cur_channel_id(), &id_3);
    assert_eq!(app_data.chat_data_ref().channels().len(), 2);

    let remove_channel_rst = app_data.chat_data_mut().remove_channel(&id_3);
    assert!(remove_channel_rst.is_ok());
    assert_eq!(app_data.chat_data_ref().cur_channel_id(), &id_2);
    assert_eq!(app_data.chat_data_ref().channels().len(), 1);

    let channel_4 = ChannelBuilder::default().name("test 4").build().unwrap();
    let id_4 = *channel_4.id();
    let add_channel_rst = app_data.chat_data_mut().add_channel(channel_4);
    assert!(add_channel_rst.is_ok());

    assert_eq!(app_data.chat_data_ref().cur_channel_id(), &id_4);
    assert_eq!(app_data.chat_data_ref().channels().len(), 2);

    let channel_5 = ChannelBuilder::default().name("test 5").build().unwrap();
    let id_5 = *channel_5.id();
    let add_channel_rst = app_data.chat_data_mut().add_channel(channel_5);
    assert!(add_channel_rst.is_ok());

    assert_eq!(app_data.chat_data_ref().cur_channel_id(), &id_5);
    assert_eq!(app_data.chat_data_ref().channels().len(), 3);

    // switch non-exist channel, has log warning
    let switch_channel_rst = app_data.chat_data_mut().switch_channel(&id_1);
    assert!(switch_channel_rst.is_ok());

    let switch_channel_rst = app_data.chat_data_mut().switch_channel(&id_4);
    assert!(switch_channel_rst.is_ok());
    let remove_channel_rst = app_data.chat_data_mut().remove_channel(&id_4);
    assert!(remove_channel_rst.is_ok());
    assert_eq!(app_data.chat_data_ref().cur_channel_id(), &id_2);

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
