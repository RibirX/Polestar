use ribir::prelude::*;

use super::app::AppGUI;

pub fn permission_page(app: impl StateWriter<Value = AppGUI>) -> impl WidgetBuilder {
  fn_widget! {
    @Row {
      @Text { text: "permission" }
      @FilledButton {
        on_tap: move |_| {
          $app.write().navigate_to("/home/chat");
        },
        @ { Label::new("switch") }
      }
    }
  }
}
