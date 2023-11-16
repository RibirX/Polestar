use ribir::{core::window::WindowId, prelude::*};

pub struct WindowInfo {
  pub id: WindowId,
  pub focused: Option<bool>,
}

pub struct WindowMgr {
  pub main: Option<WindowInfo>,
  pub quick_launcher: Option<WindowInfo>,
}

impl WindowMgr {
  pub fn new() -> Self {
    Self {
      main: None,
      quick_launcher: None,
    }
  }

  pub fn set_main_window(&mut self, id: WindowId) {
    self.main = Some(WindowInfo { id, focused: None });
  }
}

impl WindowMgr {
  pub fn dispose_quick_launcher(&mut self) {
    if let Some(window_info) = self.quick_launcher.take() {
      AppCtx::remove_wnd(window_info.id);
    }
  }
}
