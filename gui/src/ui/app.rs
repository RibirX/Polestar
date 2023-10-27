use polestar_core::model::AppData;
use ribir::prelude::*;

use super::router::*;

pub struct AppGUI {
  data: AppData,
  cur_path: String,
}

impl AppGUI {
  pub fn new(data: AppData) -> Self { Self { data, cur_path: "/hello".to_owned() } }
}

impl Compose for AppGUI {
  fn compose(this: impl StateWriter<Value = Self>) -> impl WidgetBuilder {
    fn_widget! {
      @Router {
        cur_path: pipe!($this.cur_path.clone()),
        @Route {
          path: "/hello".to_owned(),
          @Row {
            @Text { text: "hello" }
            @FilledButton {
              on_tap: move |_| {
                $this.write().cur_path = "/world".to_owned();
              },
              @ { Label::new("switch") }
            }
          }
        }
        @Route {
          path: "/world".to_owned(),
          @Row {
            @Text { text: "world" }
            @FilledButton {
              on_tap: move |_| {
                $this.write().cur_path = "/hello".to_owned();
              },
              @ { Label::new("switch") }
            }
          }
        }
      }
    }
  }
}

pub fn app_gui() -> impl WidgetBuilder {
  let app_data = AppData::new(vec![]);
  fn_widget! { @ { AppGUI::new(app_data) } }
}
