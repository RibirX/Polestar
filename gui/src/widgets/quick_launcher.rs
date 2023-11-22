use polestar_core::model::Msg;
use ribir::prelude::*;
use uuid::Uuid;

use super::app::AppGUI;

#[derive(Default)]
pub struct QuickLauncher {
  pub channel_id: Uuid,
  pub msg: Option<Msg>,
  pub status: QuickLauncherMsgStatus,
  pub selected_bot_id: Option<Uuid>,
}

impl QuickLauncher {
  pub fn new(channel_id: Uuid) -> Self { Self { channel_id, ..<_>::default() } }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum QuickLauncherMsgStatus {
  #[default]
  Done,
  Updating,
}

pub fn create_quick_launcher(app: impl StateWriter<Value = AppGUI>) {
  if app.read().quick_launcher.is_none() {
    let mut app = app.write();
    let channel_id = app.data.new_channel("Quick Launcher".to_owned(), None);
    let quick_launcher = QuickLauncher::new(channel_id);
    app.quick_launcher = Some(quick_launcher);
  }

  let quick_launcher = app.split_writer(
    |app| {
      app
        .quick_launcher
        .as_ref()
        .expect("quick launcher need existed")
    },
    |app| {
      app
        .quick_launcher
        .as_mut()
        .expect("quick launcher need existed")
    },
  );
  launcher::create_wnd(quick_launcher);
}

#[cfg(target_os = "macos")]
pub mod launcher {
  use std::time::Duration;

  use crate::{
    style::{COMMON_RADIUS, GAINSBORO_DFDFDF_FF},
    widgets::common::BotList,
    window::WindowInfo,
    WINDOW_MGR,
  };
  use bitflags::bitflags;
  use icrate::AppKit::NSApplication;
  use polestar_core::model::Msg;
  use ribir::prelude::*;

  use crate::{style::BRIGHT_GRAY_EEEDED_FF, widgets::app::AppGUI};

  use super::QuickLauncher;

  static QUICK_LAUNCHER_WND_INIT_SIZE: Size = Size::new(780., 400.);
  static QUICK_LAUNCHER_WND_SELECTED_BOT_SIZE: Size = Size::new(780., 60.);
  static QUICK_LAUNCHER_WND_CONTENT_CONTAINER_SIZE: Size = Size::new(780., 490.);
  static QUICK_LAUNCHER_CONTENT_SIZE: Size = Size::new(780., 380.);
  static ICON_SIZE: Size = Size::new(20., 20.);

  bitflags! {
    pub struct NSWindowStyleMask: usize {
      const NSWindowStyleMaskBorderless = 0;
      const NSWindowStyleMaskTitled = 1 << 0;
      const NSWindowStyleMaskClosable = 1 << 1;
      const NSWindowStyleMaskMiniaturizable = 1 << 2;
      const NSWindowStyleMaskResizable = 1 << 3;
      const NSWindowStyleMaskTexturedBackground = 1 << 8;
      const NSWindowStyleMaskUnifiedTitleAndToolbar = 1 << 12;
      const NSWindowStyleMaskFullScreen = 1 << 14;
      const NSWindowStyleMaskFullSizeContentView = 1 << 15;
      const NSWindowStyleMaskUtilityWindow = 1 << 4;
      const NSWindowStyleMaskDocModalWindow = 1 << 6;
      const NSWindowStyleMaskNonactivatingPanel = 1 << 7;
      const NSWindowStyleMaskHUDWindow = 1 << 13;
    }
  }

  impl NSWindowStyleMask {
    pub fn as_usize(&self) -> usize { self.bits() as usize }
  }

  pub fn create_wnd<S>(quick_launcher: S)
  where
    S: StateWriter<Value = QuickLauncher>,
    S::OriginWriter: StateWriter<Value = AppGUI>,
    <S::Writer as StateWriter>::OriginWriter: StateWriter<Value = AppGUI>,
    <<S::Writer as StateWriter>::Writer as StateWriter>::OriginWriter: StateWriter<Value = AppGUI>,
  {
    let wnd = App::new_window(
      w_quick_launcher(quick_launcher.clone_writer()),
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
      let style_mask = NSWindowStyleMask::NSWindowStyleMaskTitled
        | NSWindowStyleMask::NSWindowStyleMaskFullSizeContentView;
      ns_wnd.setStyleMask(style_mask.as_usize());
    }

    App::set_active_window(wnd.id());
    hide_main_window();
  }

  fn w_quick_launcher<S>(quick_launcher: S) -> impl WidgetBuilder
  where
    S: StateWriter<Value = QuickLauncher>,
    S::OriginWriter: StateWriter<Value = AppGUI>,
    <S::Writer as StateWriter>::OriginWriter: StateWriter<Value = AppGUI>,
  {
    fn_widget! {
      @ConstrainedBox {
        clamp: BoxClamp::EXPAND_BOTH,
        border_radius: Radius::all(10.),
        background: Color::from_u32(BRIGHT_GRAY_EEEDED_FF),
        @ { w_quick_launcher_editor(quick_launcher.clone_writer()) }
      }
    }
  }

  fn w_quick_launcher_editor<S>(quick_launcher: S) -> impl WidgetBuilder
  where
    S: StateWriter<Value = QuickLauncher>,
    S::OriginWriter: StateWriter<Value = AppGUI>,
  {
    let app = quick_launcher.origin_writer().clone_writer();
    fn_widget! {
      let bot_list = @BotList {
        bots: $app.data.bots_rc()
      };

      @FocusNode {
        auto_focus: false,
        @Column {
          on_key_down_capture: move |event| {
            if let PhysicalKey::Code(key_code) = event.key_code() {
              match key_code {
                KeyCode::Enter | KeyCode::NumpadEnter => {

                }
                KeyCode::KeyJ => {
                  if event.with_logo_key() {
                    let channel_id = $app.quick_launcher.as_ref().unwrap().channel_id;
                    $app.write().data.switch_channel(&channel_id);
                    hide_quick_launcher(app.clone_writer(), true);
                  }
                }
                KeyCode::KeyR => {
                  if event.with_logo_key() {

                  }
                }
                KeyCode::Backspace => {

                }
                KeyCode::Escape => {
                  hide_quick_launcher(app.clone_writer(), false);
                  event.stop_propagation();
                }
                _ => {}
              }
            }
          },
          on_key_down: move |event| {
            if let PhysicalKey::Code(key_code) = event.key_code() {
              match key_code {
                KeyCode::ArrowUp => {
                  $bot_list.write().move_up();
                }
                KeyCode::ArrowDown => {
                  $bot_list.write().move_down();
                }
                _ => {}
              }
            }
          },
          @ConstrainedBox {
            clamp: BoxClamp::fixed_height(60.),
            padding: EdgeInsets::new(0., 10., 0., 8.),
            @Row {
              @Input {
                auto_focus: true,
                @ { Placeholder::new("Type a message") }
              }
            }
          }
          @Divider {}
          @Expanded {
            flex: 1.,
            @Stack {
              fit: StackFit::Expand,
              @Visibility {
                visible: true,
                @ { bot_list }
              }
              @Visibility {
                visible: false,
                @ {
                  let msg = quick_launcher.split_writer(
                    move |quick_launcher| { quick_launcher.msg.as_ref().expect("") },
                    move |quick_launcher| { quick_launcher.msg.as_mut().expect("") },
                  );
                  w_quick_launcher_msg(msg)
                }
              }
            }
          }
        }
      }
    }
  }

  fn w_quick_launcher_msg<S>(msg: S) -> impl WidgetBuilder
  where
    S: StateWriter<Value = Msg>,
    S::OriginWriter: StateWriter<Value = QuickLauncher>,
  {
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
                  $msg.cur_cont_ref().text().map(|text| {
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
                }
              }
            }
            @Row {
              item_gap: 4.,
              align_items: Align::Center,
              right_anchor: 15.,
              bottom_anchor: 15.,
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

  pub fn create_wnd<S>(quick_launcher: S)
  where
    S: StateWriter<Value = QuickLauncher>,
    S::OriginWriter: StateWriter<Value = AppGUI>,
    <S::Writer as StateWriter>::OriginWriter: StateWriter<Value = AppGUI>,
    <<S::Writer as StateWriter>::Writer as StateWriter>::OriginWriter: StateWriter<Value = AppGUI>,
  {
  }

  pub fn hide_quick_launcher(app: impl StateWriter<Value = AppGUI>, is_need_active: bool) {}
}
