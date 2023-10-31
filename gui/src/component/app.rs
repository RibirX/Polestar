use polestar_core::model::AppData;
use ribir::prelude::*;

use super::{home::home_page, login::login_page, permission::permission_page, router::*};

pub struct AppGUI {
  cur_router_path: String,
  pub data: AppData,
}

impl AppGUI {
  pub fn new(data: AppData) -> Self {
    Self {
      data,
      cur_router_path: "/home/chat".to_owned(),
    }
  }

  pub fn cur_router_path(&self) -> &str { &self.cur_router_path }

  pub fn navigate_to(&mut self, path: &str) { self.cur_router_path = path.to_owned(); }
}

impl Compose for AppGUI {
  fn compose(this: impl StateWriter<Value = Self>) -> impl WidgetBuilder {
    fn_widget! {
      let login_page = @Route {
        path: PartialPath::new("/login", 0),
        @ { this.login_page() }
      };
      let permission_page = @Route {
        path: PartialPath::new("/permission", 0),
        @ { this.permission_page() }
      };
      let home_page = @Route {
        path: PartialPath::new("/home", 0),
        @ { this.home_page() }
      };
      @Router {
        cur_path: pipe!($this.cur_router_path().to_owned()),
        @ { login_page }
        @ { permission_page }
        @ { home_page }
      }
    }
  }
}

impl<T: StateWriter<Value = AppGUI>> AppExtraWidgets for T {}

trait AppExtraWidgets: StateWriter<Value = AppGUI> + Sized {
  fn home_page(&self) -> impl WidgetBuilder { home_page(self.clone_writer()) }

  fn login_page(&self) -> impl WidgetBuilder { login_page(self.clone_writer()) }

  fn permission_page(&self) -> impl WidgetBuilder { permission_page(self.clone_writer()) }
}

pub fn app_gui() -> impl WidgetBuilder {
  let app_data = AppData::new(vec![]);
  fn_widget! { AppGUI::new(app_data) }
}
