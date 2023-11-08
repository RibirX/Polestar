use std::marker::PhantomData;

use ribir::prelude::*;
use serde::Deserialize;

mod component;
mod style;
mod theme;

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

static G_APP_NAME: &str = "Polestar";
static G_VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
  unsafe {
    AppCtx::set_app_theme(material::purple::light());
  }

  let UISettings { window_size, language: _ } = read_ui_settings();

  let wnd = App::new_window(component::app::w_app(), Some(window_size.normal));
  wnd.set_min_size(window_size.min);
  wnd.set_title(&format!("{G_APP_NAME} v{G_VERSION}"));
  wnd.set_icon(&PixelImage::from_png(include_bytes!(
    "../static/app_logo.png"
  )));

  App::exec();
}
