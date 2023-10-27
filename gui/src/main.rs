use std::marker::PhantomData;

use ribir::prelude::*;
use serde::Deserialize;

mod component;
mod style;

#[derive(Deserialize)]
pub struct UISettings {
  pub window_size: WindowSizeConf,
  pub language: String,
}

#[derive(Deserialize)]
#[serde(remote = "Size")]
struct WndSize {
  width: f32,
  height: f32,
  #[serde(default)]
  _unit: PhantomData<LogicUnit>,
}

#[derive(Deserialize, Clone, Copy)]
pub struct WindowSizeConf {
  #[serde(with = "WndSize")]
  pub normal: Size,
  #[serde(with = "WndSize")]
  pub min: Size,
}

fn read_ui_settings() -> UISettings {
  let ui_settings: UISettings = serde_json::from_str(include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/..",
    "/gui/ui.json"
  )))
  .expect("The `ui.json` file JSON was not well-formatted");
  ui_settings
}

static APP_NAME: &str = env!("CARGO_PKG_NAME");
static VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
  unsafe {
    AppCtx::set_app_theme(material::purple::light());
  }

  let UISettings { window_size, language: _ } = read_ui_settings();

  let wnd = App::new_window(component::app::app_gui(), Some(window_size.normal));
  wnd.set_title("hello world");
  wnd.set_min_size(window_size.min);
  wnd.set_title(&format!("{APP_NAME} v{VERSION}"));
  wnd.set_icon(&PixelImage::from_png(include_bytes!(
    "../static/app_logo.png"
  )));

  App::exec();
}
