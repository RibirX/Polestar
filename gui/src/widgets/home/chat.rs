use std::rc::Rc;

use polestar_core::model::{Bot, Channel};
use ribir::prelude::*;

mod editor;
mod msg_list;
mod onboarding;

use editor::w_editor;
use msg_list::w_msg_list;

pub fn w_chat(
  channel: impl StateWriter<Value = Channel>,
  bots: Rc<Vec<Bot>>,
) -> impl WidgetBuilder {
  fn_widget! {
    @Stack {
      @Column {
        @Expanded {
          flex: 1.,
          @ { w_msg_list(channel.clone_writer()) }
        }
        @ { w_editor(channel, bots) }
      }
    }
  }
}
