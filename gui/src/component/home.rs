use ribir::prelude::*;

use super::app::AppGUI;

pub(super) fn w_home(app: impl StateWriter<Value = AppGUI>) -> impl WidgetBuilder {
  fn_widget! {
    @Row {

    }
  }
}
