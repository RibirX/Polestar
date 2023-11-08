use ribir::prelude::*;

use super::app::AppGUI;

pub(super) fn w_permission(app: impl StateWriter<Value = AppGUI>) -> impl WidgetBuilder {
  fn_widget! {
    @Row {
      @Text {
        text: "Permission".to_owned(),
      }
    }
  }
}
