use std::{cell::RefCell, rc::Rc};

use crate::req::query_open_ai;
use polestar_core::{
  model::{BotId, ChannelId, MsgAction, MsgBody},
  service::req::open_ai_request_content,
};
use ribir::prelude::*;
use uuid::Uuid;

use super::app::Chat;

pub fn send_msg(
  chat: impl StateWriter<Value = dyn Chat>,
  channel_id: ChannelId,
  msg_id: Uuid,
  idx: usize,
  bot_id: BotId,
  content: String,
  quote_id: Option<Uuid>,
) {
  let _ = AppCtx::spawn_local(async move {
    let update_msg = |act| {
      let mut chat = chat.write();
      chat.update_msg_cont(&channel_id, &msg_id, idx, act);
    };

    let quote_text = {
      quote_id.and_then(|quote_id| {
        chat.read().channel(&channel_id).and_then(|channel| {
          channel
            .msg(&quote_id)
            .and_then(|quote_msg| quote_msg.cur_cont_ref().text().map(|text| text.to_owned()))
        })
      })
    };

    let content = quote_text
      .map(|quote_text| format!("{} {}", quote_text, content))
      .unwrap_or(content);

    let text = chat
      .read()
      .channel(&channel_id)
      .map(|channel| {
        let bot = channel
          .bots()
          .and_then(|bots| bots.iter().find(|bot| bot.id() == &bot_id))
          .unwrap();
        open_ai_request_content(bot, channel, &content)
      })
      .unwrap_or(content);

    let res = query_open_ai(chat.map_reader(|chat| chat.info()), bot_id, text, |delta| {
      update_msg(MsgAction::Receiving(MsgBody::Text(Some(delta))));
    })
    .await;

    if let Err(e) = res {
      update_msg(MsgAction::Receiving(MsgBody::Text(Some(format!(
        "Error: {}",
        e
      )))));
    }
    update_msg(MsgAction::Fulfilled);
  });
}

struct LocalChannel<T>(Rc<RefCell<Vec<T>>>);
impl<T> Default for LocalChannel<T> {
  fn default() -> Self { Self(Rc::new(RefCell::new(Vec::new()))) }
}

pub(crate) trait StateSource {
  type Item;
  fn take_all(&self) -> Vec<Self::Item>;
}

pub(crate) trait StateSink {
  type Item;
  fn send(&mut self, v: Self::Item);
}

impl<T> StateSource for LocalChannel<T> {
  type Item = T;
  fn take_all(&self) -> Vec<T> { std::mem::take(&mut self.0.borrow_mut()) }
}

impl<T> StateSink for LocalChannel<T> {
  type Item = T;
  fn send(&mut self, v: T) { self.0.borrow_mut().push(v); }
}

pub(crate) fn new_channel<T: 'static + Send>() -> (
  impl StateWriter<Value = impl StateSink<Item = T>>,
  impl StateReader<Value = impl StateSource<Item = T>>,
) {
  let channel = Stateful::new(LocalChannel::default());
  (channel.clone_writer(), channel.clone_reader())
}
