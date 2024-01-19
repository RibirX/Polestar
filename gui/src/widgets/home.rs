use ribir::prelude::*;

use crate::style::{APP_SIDEBAR_WIDTH, CULTURED_F4F4F4_FF, WHITE};
use crate::widgets::home::bot_store::w_bot_store;

use super::app::{ChannelMgr, Chat, UIState, UserConfig};
use super::common::{PartialPath, Route, Router};

mod bot_store;
mod chat;
mod settings;
mod sidebar;

use chat::w_chat;
use settings::w_settings;
use sidebar::w_sidebar;

pub fn w_home(
  chat: impl StateWriter<Value = dyn Chat>,
  channel_mgr: impl StateWriter<Value = dyn ChannelMgr>,
  config: impl StateWriter<Value = dyn UserConfig>,
  ui_state: impl StateWriter<Value = dyn UIState>,
) -> impl WidgetBuilder {
  fn_widget! {
    @ {
      pipe!(!$channel_mgr.channel_ids().is_empty())
        .value_chain(|s| s.distinct_until_changed().box_it())
        .map(move |v| if v {
          let _ = || $channel_mgr.write();
          let _ = || $ui_state.write();
          @Row {
            @ConstrainedBox {
              clamp: BoxClamp::EXPAND_Y.with_fixed_width(APP_SIDEBAR_WIDTH),
              @ { w_sidebar(channel_mgr.clone_writer(), ui_state.clone_writer()) }
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
                cur_path: pipe!($ui_state.cur_path()),
                @Route {
                  path: PartialPath::new("/chat", 1),
                  @ {
                    pipe!($channel_mgr.cur_channel_id().cloned())
                      .value_chain(|s| s.distinct_until_changed().box_it())
                      .map(move |_| {
                        let channel_id = $channel_mgr.cur_channel_id().cloned().unwrap();
                        let _ = || $chat.write();
                        let def_bot_id = ($chat.info().def_bot().id()).clone();
                        w_chat(chat.clone_writer(), channel_id, $chat.info().bots_rc(), def_bot_id)
                      })
                  }
                }
                @Route {
                  path: PartialPath::new("/settings", 1),
                  @ {
                    pipe! {
                      let _ = || {
                        $config.write();
                        $ui_state.write();
                      };
                      w_settings(config.clone_writer(), ui_state.clone_writer())
                    }
                  }
                }
                @Route {
                  path: PartialPath::new("/bot_store", 1),
                  @ {
                    pipe! {
                      let _ = || {
                        $chat.write();
                        $channel_mgr.write();
                        $ui_state.write();
                      };
                      w_bot_store(
                        chat.clone_writer(),
                        channel_mgr.clone_writer(),
                        ui_state.clone_writer()
                      )
                    }
                  }
                }
              }
            }
          }.widget_build(ctx!())
        } else {
          @ { Void }.widget_build(ctx!())
        })
    }
  }
}
