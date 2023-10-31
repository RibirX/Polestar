use ribir::prelude::*;

use crate::style::{APP_SIDEBAR_WIDTH, GRAY_F4, WHITE};

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
        margin: EdgeInsets::new(10., 10., 10., 2.),
        background: Color::from_u32(WHITE),
        border_radius: Radius::all(8.),
        border: Border::all(BorderSide {
          color: Color::from_u32(GRAY_F4).into(),
          width: 1.,
        }),
        @Router {
          cur_path: pipe!($app.cur_router_path().to_owned()),
          @Route {
            path: PartialPath::new("/chat", 1),
            @ {
              let channel_writer = app.split_writer(
                |app| { app.data.cur_channel().expect("current channel must be existed") },
                |app| { app.data.cur_channel_mut().expect("current channel must be existed") },
              );
              chat(channel_writer)
            }
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
