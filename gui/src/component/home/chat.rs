use polestar_core::model::Channel;
use ribir::prelude::*;

mod editor;
mod msg_list;
mod onboarding;

use editor::w_editor;
use msg_list::w_msg_list;

pub fn w_chat(channel: impl StateWriter<Value = Channel>) -> impl WidgetBuilder {
  fn_widget! {
    @Stack {
      @Column {
        @Expanded {
          flex: 1.,
          @ { w_msg_list(channel.clone_writer()) }
        }
        @ { w_editor(channel.clone_writer()) }
      }
    }
  }
}
