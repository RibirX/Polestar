use ribir::prelude::*;

use crate::style::APP_SIDEBAR_WIDTH;

use super::{app::AppGUI, chat::chat, router::*, setting::setting, sidebar::sidebar};

pub fn home_page(app: impl StateWriter<Value = AppGUI>) -> impl WidgetBuilder {
  fn_widget! {
    @Row {
      @ConstrainedBox {
        clamp: BoxClamp::EXPAND_Y.with_fixed_width(APP_SIDEBAR_WIDTH),
        @ { sidebar(app.clone_writer()) }
      }
      @Expanded {
        flex: 1.,
        @Router {
          cur_path: pipe!($app.cur_router_path().to_owned()),
          @Route {
            path: PartialPath::new("/chat", 1),
            @ { chat(app.clone_writer()) }
          }
          @Route {
            path: PartialPath::new("/setting", 1),
            @ { setting(app.clone_writer()) }
          }
        }
      }
    }
  }
}
