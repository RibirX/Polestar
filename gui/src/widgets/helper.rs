use std::{cell::RefCell, rc::Rc};

use crate::req::query_open_ai;
use polestar_core::model::{BotId, Channel, MsgAction, MsgBody};
use ribir::prelude::*;
use uuid::Uuid;

pub fn send_msg(
  channel: impl StateWriter<Value = Channel>,
  content: String,
  idx: usize,
  msg_id: Uuid,
  bot_id: BotId,
) {
  let _ = AppCtx::spawn_local(async move {
    let update_msg = |act| {
      let mut channel = channel.write();
      channel.update_msg(&msg_id, idx, act);
    };

    let _ = query_open_ai(channel.clone_reader(), bot_id, content, |delta| {
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
