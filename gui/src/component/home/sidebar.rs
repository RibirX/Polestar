use ribir::prelude::*;

use crate::component::app::AppGUI;

pub fn w_sidebar(app: impl StateWriter<Value = AppGUI>) -> impl WidgetBuilder {
  fn_widget! {
    @Void {}
  }
}
