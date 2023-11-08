use polestar_core::model::Channel;
use ribir::prelude::*;

pub fn w_chat(channel: impl StateWriter<Value = Channel>) -> impl WidgetBuilder {
  fn_widget! {
    @Void {}
  }
}
