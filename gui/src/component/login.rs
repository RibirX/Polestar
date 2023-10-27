use ribir::prelude::*;

use super::app::AppGUI;

pub fn login_page(app: impl StateWriter<Value = AppGUI>) -> impl WidgetBuilder {
  fn_widget! {
    @Row {
      @Text { text: "login" }
      @FilledButton {
        on_tap: move |_| {
          $app.write().navigate_to("/permission");
        },
        @ { Label::new("switch") }
      }
    }
  }
}
