use std::rc::Rc;

use polestar_core::model::{
  Bot, Channel, FeedbackUserIdForServer, Msg, MsgCont, MsgMeta, MsgRole, FEEDBACK_TIMESTAMP,
};
use ribir::prelude::*;

mod editor;
mod msg_list;
mod onboarding;

use editor::w_editor;
use msg_list::w_msg_list;
use uuid::Uuid;

use crate::req::query_fetch_feedback;

pub fn w_chat(
  channel: impl StateWriter<Value = Channel>,
  bots: Rc<Vec<Bot>>,
  def_bot_id: Uuid,
) -> impl WidgetBuilder {
  fn_widget! {
    let quote_id: State<Option<Uuid>> = State::value(None);
    // handle feedback msg
    if $channel.is_feedback() {
      let channel = channel.clone_writer();
      let _ = AppCtx::spawn_local(async move {
        let data = query_fetch_feedback().await;
        let mut channel = channel.write();
        if let Ok(data) = data {
          let last_message = data.data.last();
          data.data.iter().rev().for_each(|m| {
            let msg = match m.id {
              FeedbackUserIdForServer::UserId(_) => {
                Msg::new(
                  MsgRole::User,
                  vec![MsgCont::new_text(&m.message)],
                  MsgMeta::default(),
                )
              }
              FeedbackUserIdForServer::System(id) => {
                Msg::new(
                  MsgRole::System(id),
                  vec![MsgCont::new_text(&m.message)],
                  MsgMeta::default(),
                )
              }
            };
            channel.add_msg(msg);
          });

          if let Some(last_timestamp) = data.next {
            *FEEDBACK_TIMESTAMP.lock().unwrap() = Some(last_timestamp);
          } else if let Some(last_message) = last_message {
            *FEEDBACK_TIMESTAMP.lock().unwrap() = Some(last_message.create_time);
          }
        }
      });
    }
    @Stack {
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
