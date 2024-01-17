use ribir::prelude::*;
use uuid::Uuid;

use super::app::AppGUI;

#[derive(Default, Clone)]
pub struct QuickLauncher {
  pub channel_id: Uuid,
  pub msg: Option<Uuid>,
  pub selected_bot_id: Option<Uuid>,
}

impl QuickLauncher {
  pub fn new(channel_id: Uuid) -> Self { Self { channel_id, ..<_>::default() } }
}

pub fn create_quick_launcher(app: impl StateWriter<Value = AppGUI>) {
  // if app.read().quick_launcher.is_none() {
  //   let mut app = app.write();
  //   let channel_id = app
  //     .data
  //     .new_channel("Quick Launcher".to_owned(), None, ChannelCfg::default());
  //   app.data.info_mut().set_quick_launcher_id(Some(channel_id));
  //   let quick_launcher = QuickLauncher::new(channel_id);
  //   app.quick_launcher = Some(quick_launcher);
  // }

  // launcher::create_wnd(app.clone_writer());
}

#[cfg(target_os = "macos")]
pub mod launcher {
  use std::time::Duration;

  use crate::{
    style::{COMMON_RADIUS, GAINSBORO_DFDFDF_FF, WHITE},
    theme::polestar_svg,
    widgets::common::{w_avatar, BotList, IconButton},
    window::WindowInfo,
    WINDOW_MGR,
  };
  use icrate::AppKit::{
    NSApplication, NSWindowStyleMaskFullSizeContentView, NSWindowStyleMaskTitled,
  };
  use polestar_core::{
    model::{Msg, MsgAction, MsgBody, MsgCont, MsgMeta, MsgRole},
    service::open_ai::mock_stream_string,
  };
  use ribir::prelude::*;
  use uuid::Uuid;

  use crate::widgets::app::AppGUI;

  static QUICK_LAUNCHER_WND_INIT_SIZE: Size = Size::new(780., 400.);
  static QUICK_LAUNCHER_WND_SELECTED_BOT_SIZE: Size = Size::new(780., 60.);
  static QUICK_LAUNCHER_CONTENT_SIZE: Size = Size::new(780., 380.);
  static ICON_SIZE: Size = Size::new(20., 20.);

  fn set_quick_launcher_size(size: Size) {
    let wnd_manager = WINDOW_MGR.lock().unwrap();
    let wnd = wnd_manager.quick_launcher.as_ref().unwrap();
    if let Some(wnd) = AppCtx::get_window(wnd.id) {
      wnd.shell_wnd().borrow_mut().request_resize(size);
    }
  }

  pub fn create_wnd(app: impl StateWriter<Value = AppGUI>) {
    let wnd = App::new_window(
      w_quick_launcher(app.clone_writer()),
      Some(QUICK_LAUNCHER_WND_INIT_SIZE),
    );

    wnd.shell_wnd().borrow_mut().set_decorations(false);

    {
      let mut window_mgr = WINDOW_MGR.lock().unwrap();
      window_mgr.quick_launcher = Some(WindowInfo { id: wnd.id(), focused: None });
    }

    unsafe {
      let ns_app = NSApplication::sharedApplication();
      // get the quick launcher window, it's index is 1.
      let ns_wnd = ns_app.windows().objectAtIndex(1);
      ns_wnd.setTitlebarAppearsTransparent(true);
      ns_wnd.setTitleVisibility(1);
      if let Some(close_btn) = ns_wnd.standardWindowButton(0) {
        close_btn.setHidden(true);
      }
      if let Some(minimize_btn) = ns_wnd.standardWindowButton(1) {
        minimize_btn.setHidden(true);
      }
      ns_wnd.setOpaque(false);
      let style_mask = NSWindowStyleMaskTitled | NSWindowStyleMaskFullSizeContentView;
      ns_wnd.setStyleMask(style_mask);
    }

    App::set_active_window(wnd.id());
    hide_main_window();
  }

  fn w_quick_launcher(app: impl StateWriter<Value = AppGUI>) -> impl WidgetBuilder {
    fn_widget! {
      @ConstrainedBox {
        clamp: BoxClamp::EXPAND_BOTH,
        border_radius: Radius::all(10.),
        background: Color::from_u32(WHITE),
        @ { w_quick_launcher_editor(app.clone_writer()) }
      }
    }
  }

  fn w_quick_launcher_editor(app: impl StateWriter<Value = AppGUI>) -> impl WidgetBuilder {
    fn_widget! {
      let bot_list = @BotList {
        bots: $app.data.info().bots_rc(),
        visible: pipe! {
          $app.quick_launcher.as_ref().map(|quick_launcher| {
            quick_launcher.selected_bot_id.is_none() && quick_launcher.msg.is_none()
          }).unwrap_or_default()
        }
      };

      let input = @Input {
        auto_focus: true,
      };

      let send_icon = @IconButton {
        on_tap: move |_| {

        },
        @ { polestar_svg::SEND }
      };

      let msg_visibility = @Visibility {
        visible: pipe!($app.has_quick_launcher_msg())
      };

      let quick_launcher_app = app.clone_writer();

      @FocusNode {
        auto_focus: true,
        @Column {
          on_key_down_capture: move |e| {
            match e.key_code() {
              PhysicalKey::Code(KeyCode::Enter | KeyCode::NumpadEnter) => {
                if $input.text().is_empty() {
                  if let Some((bot_id, _)) = $bot_list.selected_bot() {
                    if let Some(quick_launcher) = $app.write().quick_launcher.as_mut() {
                      quick_launcher.selected_bot_id = Some(bot_id);
                    }
                    e.stop_propagation();
                  }
                } else {
                  let input = $input;
                  let text = input.text();
                  // create msg to channel.
                  let msg = Msg::new_user_text(text, MsgMeta::default());
                  let id = *msg.id();

                  let mut app_write_ref = $app.write();
                  let quick_launcher_id = app_write_ref.quick_launcher_id().expect("quick launcher id is none");
                  let channel = app_write_ref.data.get_channel_mut(&quick_launcher_id).unwrap();

                  channel.add_msg(msg);
                  // receive msg from response.

                  // TODO: get bot id
                  let bot_id = Uuid::nil();
                  let mut msg = Msg::new_bot_text(bot_id, MsgMeta::reply(id));
                  msg.cur_cont_mut().action(MsgAction::Receiving(MsgBody::Text(Some("Hi".to_owned()))));
                  let id = *msg.id();
                  channel.add_msg(msg);

                  if let Some(msg) = channel.msg_mut(&id) {
                    let cur_cont = msg.cur_cont_mut();
                    mock_stream_string("", |delta| {
                      cur_cont.action(MsgAction::Receiving(MsgBody::Text(Some(delta.to_string()))));
                    });
                  }

                  if let Some(quick_launcher) = app_write_ref.quick_launcher.as_mut() {
                    quick_launcher.msg = Some(id);
                  }

                  e.stop_propagation();
                }
              }
              PhysicalKey::Code(KeyCode::KeyJ) => {
                if e.with_logo_key() {
                  let channel_id = $app.quick_launcher.as_ref().unwrap().channel_id;
                  $app.write().data.switch_channel(&channel_id);
                  hide_quick_launcher(app.clone_writer(), true);
                }
              }
              PhysicalKey::Code(KeyCode::KeyR) => {
                if e.with_logo_key() {

                }
              }
              PhysicalKey::Code(KeyCode::Backspace) => {}
              PhysicalKey::Code(KeyCode::Escape) => {
                hide_quick_launcher(app.clone_writer(), false);
                e.stop_propagation();
              }
              _ => {}
            }
          },
          on_key_down: move |e| {
            match e.key_code() {
              PhysicalKey::Code(KeyCode::ArrowUp) => {
                $bot_list.write().move_up();
              }
              PhysicalKey::Code(KeyCode::ArrowDown) => {
                $bot_list.write().move_down();
              }
              _ => {}
            }
          },
          @ConstrainedBox {
            clamp: BoxClamp::fixed_height(60.),
            padding: EdgeInsets::new(0., 10., 0., 8.),
            @Row {
              @ {
                pipe!{
                  $app.quick_launcher.as_ref().and_then(|quick_launcher| {
                    quick_launcher.selected_bot_id
                  })
                }.map(move |id| {
                  let app = app.clone_writer();
                  id.and_then(move |id| {
                    set_quick_launcher_size(QUICK_LAUNCHER_WND_SELECTED_BOT_SIZE);
                    let app_ref = $app;
                    let bot = app_ref.data.info().bots().iter().find(|bot| bot.id() == &id);
                    bot.map(|bot| {
                      w_avatar(bot.avatar().clone())
                    })
                  })
                })
              }
              @$input { @ { Placeholder::new("Type a message") } }
              @ { send_icon }
            }
          }
          @Divider {}
          @Expanded {
            flex: 1.,
            @Stack {
              fit: StackFit::Expand,
              @ { bot_list }
              @$msg_visibility {
                @ {
                  w_quick_launcher_msg(quick_launcher_app.clone_writer())
                }
              }
            }
          }
        }
      }
    }
  }

  fn w_quick_launcher_msg(app: impl StateWriter<Value = AppGUI>) -> impl WidgetBuilder {
    fn_widget! {
      @Column {
        padding: EdgeInsets::new(10., 8., 10., 8.),
        @Container {
          size: QUICK_LAUNCHER_CONTENT_SIZE,
          background: Color::from_u32(GAINSBORO_DFDFDF_FF),
          border_radius: COMMON_RADIUS,
          @Stack {
            @VScrollBar {
              margin: EdgeInsets::new(12., 12., 40., 12.),
              @Column {
                @ {
                  pipe! {
                    $app.quick_launcher.as_ref().and_then(|quick_launcher| {
                      quick_launcher.msg.as_ref().and_then(|msg_id| {
                        set_quick_launcher_size(QUICK_LAUNCHER_CONTENT_SIZE);
                        let app_ref = $app;
                        let quick_launcher_id = app_ref.quick_launcher.as_ref().unwrap().channel_id;
                        let channel = app_ref.data.get_channel(&quick_launcher_id).unwrap();
                        channel.msg(msg_id).and_then(|msg| {
                          msg.cur_cont_ref().text().map(|text| {
                            @TextSelectable {
                              cursor: CursorIcon::Text,
                              @Text {
                                text: text.to_owned(),
                                foreground: Palette::of(ctx!()).on_background(),
                                overflow: Overflow::AutoWrap,
                                text_style: TypographyTheme::of(ctx!()).body_medium.text.clone(),
                              }
                            }
                          })
                        })
                      })
                    })
                  }
                }
              }
            }
            @Row {
              item_gap: 4.,
              align_items: Align::Center,
              anchor: Anchor::right_bottom(15., 15.),
              @Text {
                text: "Copy to clipboard",
              }
              @ { w_label_button("⏎") }
              @Text { text: " | " }
              @Text { text: "Retry" }
              @ { w_label_button("⌘") }
              @ { w_label_button("R") }
            }
          }
        }
        @Row {
          justify_content: JustifyContent::SpaceBetween,
          margin: EdgeInsets::only_top(10.),
          @Row {
            align_items: Align::Center,
            item_gap: 4.,
            @Text {
              text: "Back",
            }
            @ { w_label_button("Esc") }
          }
          @Row {
            align_items: Align::Center,
            item_gap: 4.,
            @Text {
              text: "Continue in chat",
            }
            @ { w_label_button("⌘") }
            @ { w_label_button("J") }
          }
        }

      }
    }
  }

  fn w_label_button(str: &str) -> impl WidgetBuilder + '_ {
    fn_widget! {
      @SizedBox {
        size: ICON_SIZE,
        border_radius: Radius::all(4.),
        background: Color::from_u32(GAINSBORO_DFDFDF_FF),
        @Text {
          h_align: HAlign::Center,
          text: str.to_owned()
        }
      }
    }
  }

  pub fn hide_quick_launcher(app: impl StateWriter<Value = AppGUI>, is_need_active: bool) {
    let is_hidden = hide_app();
    let app = app.clone_writer();

    timer(is_hidden, Duration::from_millis(100), AppCtx::scheduler()).subscribe(move |is_hidden| {
      app.write().quick_launcher = None;
      let mut window_mgr = WINDOW_MGR.lock().unwrap();
      window_mgr.dispose_quick_launcher();

      window_mgr.main.as_ref().map(|wnd_info| {
        if let Some(wnd) = AppCtx::get_window(wnd_info.id) {
          wnd.shell_wnd().borrow_mut().set_visible(true);
        }
      });

      if !is_hidden {
        unsafe {
          let ns_app = NSApplication::sharedApplication();
          ns_app.activateIgnoringOtherApps(is_need_active);
        }
      }
    });
  }

  fn hide_main_window() {
    let mut window_mgr = WINDOW_MGR.lock().unwrap();
    if let Some(wnd_info) = window_mgr.main.as_ref() {
      if let Some(wnd) = AppCtx::get_window(wnd_info.id) {
        wnd.shell_wnd().borrow_mut().set_visible(false);
      }
    }
  }

  fn hide_app() -> bool {
    unsafe {
      let ns_app = NSApplication::sharedApplication();
      let is_hidden = ns_app.isHidden();
      ns_app.hide(None);
      is_hidden
    }
  }
}

#[cfg(target_os = "windows")]
pub mod launcher {
  use super::QuickLauncher;
  use crate::widgets::app::AppGUI;
  use ribir::prelude::*;

  pub fn create_wnd(app: impl StateWriter<Value = AppGUI>) {}

  pub fn hide_quick_launcher(app: impl StateWriter<Value = AppGUI>, is_need_active: bool) {}
}
