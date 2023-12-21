use std::rc::Rc;

use polestar_core::model::{Bot, Channel};
use ribir::prelude::*;

mod editor;
mod msg_list;
mod onboarding;

use editor::w_editor;
use msg_list::w_msg_list;
use uuid::Uuid;

pub fn w_chat(
  channel: impl StateWriter<Value = Channel>,
  bots: Rc<Vec<Bot>>,
  def_bot_id: Uuid,
) -> impl WidgetBuilder {
  fn_widget! {
    let quote_id = State::value(None) as State<Option<Uuid>>;
    @Stack {
      @Column {
        @Expanded {
          flex: 1.,
          @ { w_msg_list(channel.clone_writer(), quote_id.clone_writer()) }
        }
        @ { w_editor(channel, bots, def_bot_id, quote_id.clone_writer()) }
      }
    }
  }
}
