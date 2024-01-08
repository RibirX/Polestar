use std::{cell::RefCell, rc::Rc};

use crate::req::query_open_ai;
use polestar_core::model::{Bot, BotId, Channel, MsgAction, MsgBody};
use ribir::prelude::*;
use uuid::Uuid;

pub fn send_msg(
  channel: impl StateWriter<Value = Channel>,
  content: String,
  idx: usize,
  msg_id: Uuid,
  bot_id: BotId,
) {
  let (url, headers) = {
    let channel = channel.read();
    let bot = channel
      .bots()
      .and_then(|bots: &[Bot]| bots.iter().find(|bot| bot.id() == &bot_id));
    bot
      .map(|bot| {
        (
          bot.url().to_string(),
          bot.headers().try_into().ok().unwrap_or_default(),
        )
      })
      .unwrap_or_default()
  };

  let _ = AppCtx::spawn_local(async move {
    let update_msg = |act| {
      let mut channel = channel.write();
      channel.update_msg(&msg_id, idx, act);
    };

    let _ = query_open_ai(url, content, headers, |delta| {
      update_msg(MsgAction::Receiving(MsgBody::Text(Some(delta))));
    })
    .await;

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
