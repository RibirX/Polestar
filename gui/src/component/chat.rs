use ribir::prelude::*;

use super::app::AppGUI;

pub fn chat(app: impl StateWriter<Value = AppGUI>) -> impl WidgetBuilder {
  fn_widget! {
    @Column {
      @Text { text: "chat" }
      @FilledButton {
        on_tap: move |_| {
          $app.write().navigate_to("/home/setting");
        },
        @ { Label::new("switch setting") }
      }
    }
  }
}
