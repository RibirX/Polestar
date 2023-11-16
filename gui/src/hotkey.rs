#[cfg(target_os = "macos")]
pub mod handler {
  use icrate::AppKit::NSApplication;
  use ribir::core::window::WindowId;
  use ribir::prelude::*;

  use crate::component::app::AppGUI;
  use crate::component::{hide_quick_launcher, create_quick_launcher, QuickLauncher};
  use crate::WINDOW_MGR;

  pub fn focus_handler(
    app: impl StateWriter<Value = AppGUI>,
    wnd_id: &mut WindowId,
    focused: &mut bool,
  ) {
    let mut window_mgr = WINDOW_MGR.try_lock().unwrap();
    let quick_launcher = window_mgr.quick_launcher.as_ref();
    if Some(*wnd_id) == quick_launcher.map(|wnd_info| wnd_info.id) {
      if !*focused && quick_launcher.and_then(|wnd| wnd.focused).is_some() {
        let is_hidden = unsafe {
          let ns_app = NSApplication::sharedApplication();
          ns_app.isHidden()
        };
        if !is_hidden {
          hide_quick_launcher(app.clone_writer(), false);
        }
      }

      let quick_launcher = window_mgr.quick_launcher.as_mut();
      quick_launcher.map(|wnd| wnd.focused = Some(*focused));
    }
    let main = window_mgr.main.as_mut();
    if Some(*wnd_id) == main.as_ref().map(|wnd| wnd.id) {
      main.map(|wnd| wnd.focused = Some(*focused));
    }
  }

  pub fn hotkey_handler(hotkey: &HotkeyEvent, app: impl StateWriter<Value = AppGUI>) {
    if hotkey.modifiers == Some(ModifiersState::SUPER | ModifiersState::ALT) {
      if hotkey.key_code == Some(KeyCode::KeyK) {
        create_quick_launcher(app.clone_writer())
      }
    }
  }

  
}

#[cfg(target_os = "windows")]
pub mod handler {
  #[allow(unused)]
  pub fn hotkey_handler(_hotkey: &HotkeyEvent, _app: impl StateWriter<Value = AppGUI>) {}

  #[allow(unused)]
  pub fn focus_handler(app: impl StateWriter<Value = AppGUI>) {}
}
