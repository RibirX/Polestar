use ribir::prelude::*;

use crate::{
  component::app::AppGUI,
  style::{COMMON_RADIUS, WHITE},
};

pub fn w_settings(app: impl StateWriter<Value = AppGUI>) -> impl WidgetBuilder {
  fn_widget! {
    @Void {}
  }
}
