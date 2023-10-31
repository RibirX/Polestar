use polestar_core::model::Channel;
use ribir::prelude::*;

mod chat_list;
mod editor;

pub fn chat(channel: impl StateWriter<Value = Channel>) -> impl WidgetBuilder {
  fn_widget! {
    @Stack {
      @Column {
        @Expanded {
          flex: 1.,
          @ { chat_list::chat_list() }
        }
        @ { editor::editor() }
      }
    }
  }
}
