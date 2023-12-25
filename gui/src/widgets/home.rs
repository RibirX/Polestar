use ribir::prelude::*;

use crate::style::{APP_SIDEBAR_WIDTH, CULTURED_F4F4F4_FF, WHITE};
use crate::widgets::home::bot_store::w_bot_store;

use super::app::AppGUI;
use super::common::{PartialPath, Route, Router};

mod bot_store;
mod chat;
mod settings;
mod sidebar;

use chat::w_chat;
use settings::w_settings;
use sidebar::w_sidebar;

pub fn w_home(app: impl StateWriter<Value = AppGUI>) -> impl WidgetBuilder {
  fn_widget! {
    @Row {
      @ConstrainedBox {
        clamp: BoxClamp::EXPAND_Y.with_fixed_width(APP_SIDEBAR_WIDTH),
        @ { w_sidebar(app.clone_writer()) }
      }
      @Expanded {
        flex: 1.,
        margin: EdgeInsets::new(10., 10., 10., 2.),
        background: Color::from_u32(WHITE),
        border_radius: Radius::all(8.),
        border: Border::all(BorderSide {
          color: Color::from_u32(CULTURED_F4F4F4_FF).into(),
          width: 1.,
        }),
        @Router {
          cur_path: pipe!($app.cur_router_path().to_owned()),
          @Route {
            path: PartialPath::new("/chat", 1),
            @ {
              pipe! {
                let _ = || $app.write();
                let def_bot_id = *($app.data.info().def_bot().id());
                let channel_writer = app.split_writer(
                  |app| { app.data.cur_channel().expect("current channel must be existed") },
                  |app| { app.data.cur_channel_mut().expect("current channel must be existed") },
                );
                w_chat(channel_writer.clone_writer(), $app.data.info().bots_rc(), def_bot_id)
              }
            }
          }
          @Route {
            path: PartialPath::new("/settings", 1),
            @ {
              pipe! {
                let _ = || $app.write();
                w_settings(app.clone_writer())
              }
            }
          }
          @Route {
            path: PartialPath::new("/bot_store", 1),
            @ {
              pipe! {
                let _ = || $app.write();
                w_bot_store(app.clone_writer())
              }
            }
          }
        }
      }
    }
  }
}
