use ribir::prelude::*;

use super::app::AppGUI;

pub fn channel(_app: impl StateWriter<Value = AppGUI>) -> impl WidgetBuilder {
  fn_widget! {
    @Text { text: "channel" }
  }
}
