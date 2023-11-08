use polestar_core::{load_bot_cfg_file, model::AppData};
use ribir::prelude::*;
use ribir_algo::Sc;

use super::{
  common::{PartialPath, Route, Router, Tooltip},
  home::w_home,
  login::w_login,
  permission::w_permission,
};
use crate::theme::polestar_theme;

pub struct AppGUI {
  cur_router_path: String,
  pub data: AppData,
}

impl AppGUI {
  pub fn new(data: AppData) -> Self {
    Self {
      data,
      // TODO: launch need confirm current router by user login status.
      // It can get by open app url, like this: PoleStarChat://ribir.org/home/chat
      cur_router_path: "/home/chat".to_owned(),
    }
  }

  pub fn cur_router_path(&self) -> &str { &self.cur_router_path }

  pub fn navigate_to(&mut self, path: &str) { self.cur_router_path = path.to_owned(); }
}

impl Compose for AppGUI {
  fn compose(this: impl StateWriter<Value = Self>) -> impl WidgetBuilder {
    fn_widget! {
      @ThemeWidget {
        // Polestar custom theme.
        theme: Sc::new(Theme::Inherit(polestar_theme())),
        @ {
          Box::new(fn_widget! {
            @Stack {
              @Router {
                cur_path: pipe!($this.cur_router_path().to_owned()),
                @Route {
                  path: PartialPath::new("/login", 0),
                  @ { w_login(this.clone_writer()) }
                }
                @Route {
                  path: PartialPath::new("/permission", 0),
                  @ { w_permission(this.clone_writer()) }
                }
                @Route {
                  path: PartialPath::new("/home", 0),
                  @ { w_home(this.clone_writer()) }
                }
              }
              @Tooltip {
                content: "hello world".to_owned(),
              }
            }
          })
        }
      }
    }
  }
}

impl<T: StateWriter<Value = AppGUI>> AppExtraWidgets for T {}

trait AppExtraWidgets: StateWriter<Value = AppGUI> + Sized {
  // TODO: others data operators.
}

// launch App need to do some init work.
// 1. [x] load bot config file.
// 2. [ ] if has user module, need check user login status.
// 3. [ ] some static file can't be load dynamic, need load as bytes before
//    launch.
// 4. [ ] load local db data.
pub fn w_app() -> impl WidgetBuilder {
  let bots = load_bot_cfg_file();
  let first_bot_id = *bots.first().unwrap().id();
  let mut app_data = AppData::new(bots, first_bot_id);
  app_data.new_channel("quick start".to_owned(), None);

  // XXX: how to build `AppGUI` global state?
  // 1. If I create global state `Stateful<AppGUI>`, `fn_widget!` need capture
  //    this global state ownership. Like this global state is pointless.
  // 2. If I create global state `AppGUI`, `Stateful<&mut AppGUI>` can't compiler.

  fn_widget! { AppGUI::new(app_data) }
}
