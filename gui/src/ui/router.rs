use ribir::prelude::*;

#[derive(Template)]
pub struct Route {
  path: String,
  component: WidgetOf<RouteItem>,
}

#[derive(Declare, PairChild)]
pub struct RouteItem;

#[derive(Declare)]
pub struct Router {
  cur_path: String,
}

impl ComposeChild for Router {
  type Child = Vec<Route>;
  fn compose_child(this: impl StateWriter<Value = Self>, child: Self::Child) -> impl WidgetBuilder {
    fn_widget! {
      @Stack {
        @ {
          child.into_iter().map(|route| @Visibility {
            visible: route.path == $this.cur_path,
            @ { route.component.child() }
          })
        }
      }
    }
  }
}
