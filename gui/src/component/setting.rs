use ribir::prelude::*;

use super::app::AppGUI;

pub fn setting(app: impl StateWriter<Value = AppGUI>) -> impl WidgetBuilder {
  fn_widget! {
    @Column {
      @Text { text: "setting" }
      @FilledButton {
        on_tap: move |_| {
          $app.write().navigate_to("/home/chat");
        },
        @ { Label::new("switch chat") }
      }
    }
  }
}
