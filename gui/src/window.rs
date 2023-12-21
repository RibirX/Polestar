use ribir::{core::window::WindowId, prelude::*};

pub struct WindowInfo {
  pub id: WindowId,
  pub focused: Option<bool>,
}

#[derive(Default)]
pub struct WindowMgr {
  pub main: Option<WindowInfo>,
  pub quick_launcher: Option<WindowInfo>,
}

impl WindowMgr {
  pub fn set_main_window(&mut self, id: WindowId) {
    self.main = Some(WindowInfo { id, focused: None });
  }
}

impl WindowMgr {
  pub fn dispose_quick_launcher(&mut self) {
    if let Some(window_info) = self.quick_launcher.take() {
      println!("dispose quick launcher: {:?}", window_info.id);
      AppCtx::remove_wnd(window_info.id);
    }
  }
}
