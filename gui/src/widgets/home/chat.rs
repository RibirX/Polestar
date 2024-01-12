use std::rc::Rc;

use polestar_core::model::{
  Bot, BotId, Channel, FeedbackUserIdForServer, Msg, MsgCont, MsgMeta, MsgRole,
};
use ribir::prelude::*;

mod editor;
mod msg_list;
mod onboarding;

use editor::w_editor;
use msg_list::w_msg_list;
use uuid::Uuid;

use crate::{
  req::query_fetch_feedback,
  widgets::helper::{new_channel, StateSink, StateSource},
};

pub fn w_chat(
  channel: impl StateWriter<Value = Channel>,
  bots: Rc<Vec<Bot>>,
  def_bot_id: BotId,
) -> impl WidgetBuilder {
  fn_widget! {
    let quote_id: State<Option<Uuid>> = State::value(None);
    let mut guard = None;
    if $channel.is_feedback() {
      let (sx, rx) = new_channel();
      let mut time_stamp = $channel.msgs().last().map(|msg| msg.create_at().timestamp_millis());
      fetch_feedbacks(sx, time_stamp);
      guard = Some(watch!($rx.take_all()).subscribe(move |feedbacks|{
        feedbacks.into_iter().for_each(|msg| {
          let t = msg.create_at().timestamp_millis();
          if time_stamp.is_none() || t > time_stamp.unwrap() {
            time_stamp = Some(t);
            $channel.write().add_msg(msg);
          }
        });
      }).unsubscribe_when_dropped());
    }
    @Stack {
      on_disposed: move |_| { guard.take(); },
      @Column {
        @Expanded {
          flex: 1.,
          @ { w_msg_list(channel.clone_writer(), quote_id.clone_writer()) }
        }
        @ { w_editor(channel, bots, def_bot_id, quote_id) }
      }
    }
  }
}

fn fetch_feedbacks(
  sender: impl StateWriter<Value = impl StateSink<Item = Msg>>,
  utc_time: Option<i64>,
) {
  let _ = AppCtx::spawn_local(async move {
    let data = query_fetch_feedback(utc_time).await;

    if let Ok(data) = data {
      data.data.iter().rev().for_each(|m| {
        let msg = match m.id {
          FeedbackUserIdForServer::UserId(_) => Msg::new(
            MsgRole::User,
            vec![MsgCont::new_text(&m.message)],
            MsgMeta::default(),
            Some(m.create_time as i64),
          ),
          FeedbackUserIdForServer::System(id) => Msg::new(
            MsgRole::System(id),
            vec![MsgCont::new_text(&m.message)],
            MsgMeta::default(),
            Some(m.create_time as i64),
          ),
        };
        sender.write().send(msg);
      })
    }
  });
}
