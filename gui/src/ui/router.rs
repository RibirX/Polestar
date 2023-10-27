use ribir::prelude::*;

#[derive(Declare)]
pub struct Route {
  path: String,
}

impl ComposeChild for Route {
  type Child = Widget;
  fn compose_child(_: impl StateWriter<Value = Self>, child: Self::Child) -> impl WidgetBuilder {
    fn_widget! { child }
  }
}

#[derive(Declare)]
pub struct Router {
  cur_path: String,
}

impl ComposeChild for Router {
  type Child = Vec<Pair<State<Route>, Widget>>;
  fn compose_child(this: impl StateWriter<Value = Self>, child: Self::Child) -> impl WidgetBuilder {
    fn_widget! {
      @Stack {
        @ {
          child.into_iter().map(|p| {
            let (route, child) = p.unzip();
            let  path = $route.path.clone();
            @Visibility {
              visible: pipe!(path == $this.cur_path),
              @ { child }
            }
          })
        }
      }
    }
  }
}
