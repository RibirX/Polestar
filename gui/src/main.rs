use std::{marker::PhantomData, sync::Mutex, time::Instant};

use once_cell::sync::Lazy;
use ribir::prelude::*;
use serde::Deserialize;

mod oauth;
mod platform;
mod req;
mod style;
mod theme;
mod widgets;

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

#[derive(Default)]
pub struct GlobalConfig {
  pub local_server_port: Option<u16>,
}

pub static GLOBAL_CONFIG: Lazy<Mutex<GlobalConfig>> =
  Lazy::new(|| Mutex::new(GlobalConfig::default()));

pub static TIMER: Lazy<Mutex<Instant>> = Lazy::new(|| Mutex::new(Instant::now()));

fn main() {
  if !platform::app_init_hook() {
    return;
  }

  local_server_listen();
  unsafe {
    AppCtx::set_app_theme(material::purple::light());
  }
  install_fonts();
  let UISettings { window_size, language: _ } = read_ui_settings();

  let wnd = App::new_window(widgets::app::w_app(), Some(window_size.normal));
  wnd.set_min_size(window_size.min);
  wnd.set_title(&format!("{G_APP_NAME} v{G_VERSION}"));
  wnd.set_icon(&PixelImage::from_png(include_bytes!(
    "../static/app_logo.png"
  )));

  *TIMER.lock().unwrap() = Instant::now();

  platform::app_run_before_hook();

  App::exec();
}

fn local_server_listen() {
  use base64::{engine::general_purpose, Engine as _};
  use tiny_http::{Request, Response, Server};
  use url::Url;
  fn login_handle(url: Url, req: Request, event_sender: &EventSender) {
    if let Some(pair) = url.query_pairs().find(|(key, _)| key == "url") {
      if let Some(url) = general_purpose::STANDARD
        .decode(pair.1.as_bytes())
        .ok()
        .and_then(|content| String::from_utf8(content).ok())
      {
        event_sender.send(AppEvent::OpenUrl(url));
        let _ = req.respond(Response::from_string("Ok"));
        return;
      }
    }
    let _ = req.respond(Response::from_string("failed").with_status_code(400));
  }

  let event_sender = App::event_sender();
  std::thread::spawn(move || {
    let local_server_port = 56765;
    GLOBAL_CONFIG.lock().unwrap().local_server_port = Some(local_server_port);
    let server = Server::http(format!("127.0.0.1:{local_server_port}"));
    let Ok(server) = server else {
      println!("server launch failed {:?}", server.err());
      return;
    };

    for request in server.incoming_requests() {
      if let Ok(url) = url::Url::parse(&format!("http://127.0.0.1{}", request.url())) {
        match url.path() {
          "/login" => login_handle(url, request, &event_sender),
          _ => {
            let _ = request.respond(Response::from_string("not found").with_status_code(404));
          }
        }
      }
    }
  });
}

fn install_fonts() {
  let font_db = AppCtx::font_db();
  [
    include_bytes!("./theme/fonts/Inter-Regular.otf").to_vec(),
    include_bytes!("./theme/fonts/Inter-Bold.otf").to_vec(),
    include_bytes!("./theme/fonts/Inter-Medium.otf").to_vec(),
    include_bytes!("./theme/fonts/NotoSansSC-Regular.otf").to_vec(),
    include_bytes!("./theme/fonts/NotoSansSC-Bold.otf").to_vec(),
    include_bytes!("./theme/fonts/NotoSansSC-Medium.otf").to_vec(),
    include_bytes!("./theme/fonts/NotoColorEmoji.ttf").to_vec(),
  ]
  .into_iter()
  .for_each(|font| font_db.borrow_mut().load_from_bytes(font));

  font_db.borrow_mut().set_default_fonts(&FontFace {
    families: Box::new([
      FontFamily::Name("Inter".into()),
      FontFamily::Name("PingFang SC".into()),
      FontFamily::Name("Segoe UI".into()),
      FontFamily::Name("Helvetica".into()),
      FontFamily::Name("Arial".into()),
      FontFamily::Name("Noto Sans SC".into()),
      FontFamily::Name("Noto Color Emoji".into()),
    ]),
    ..Default::default()
  });
}
