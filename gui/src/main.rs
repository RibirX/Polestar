use ribir::prelude::*;

mod ui;

fn main() {
  unsafe {
    AppCtx::set_app_theme(material::purple::light());
  }
  App::new_window(ui::app::app_gui(), None).set_title("hello world");
  App::exec();
}
