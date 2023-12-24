use crate::req::query_open_ai;
use polestar_core::model::{Bot, Channel, MsgAction, MsgBody};
use ribir::prelude::*;
use uuid::Uuid;

pub fn send_msg(
  channel: impl StateWriter<Value = Channel>,
  content: String,
  idx: usize,
  msg_id: Uuid,
  bot_id: Uuid,
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
    if channel.read().msg(&msg_id).is_none() {
      return;
    }

    let update_msg = |act| {
      let mut channel = channel.write();
      let cur_cont = channel.msg_mut(&msg_id).unwrap().cont_mut(idx);
      cur_cont.action(act);
    };

    query_open_ai(url, content, headers, |delta| {
      update_msg(MsgAction::Receiving(MsgBody::Text(Some(delta))));
    })
    .await;

    update_msg(MsgAction::Fulfilled);
  });
}
