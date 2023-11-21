use ribir::prelude::*;

#[derive(Declare)]
pub struct Route {
  path: PartialPath,
}

#[derive(Clone)]
pub struct PartialPath {
  partial: String,
  level: usize,
}

impl PartialPath {
  pub fn new(partial: impl Into<String>, level: usize) -> Self {
    Self { partial: partial.into(), level }
  }

  pub fn is_match(&self, path: &str) -> bool {
    if path.chars().nth(0) == Some('/') {
      let p = path[1..].split('/').nth(self.level);
      // partial path must be equal to the path or be a param as `:param`
      return self.partial.chars().nth(1) == Some(':')
        || p == Some(self.partial.as_str()[1..].as_ref());
    }
    false
  }

  pub fn get_param(&self, path: &str) -> Option<String> {
    if path.chars().nth(0) == Some('/') && self.partial.chars().nth(1) == Some(':') {
      return path[1..].split('/').nth(self.level).map(|s| s.to_owned());
    }
    None
  }
}

impl ComposeChild for Route {
  type Child = Widget;
  fn compose_child(_: impl StateWriter<Value = Self>, child: Self::Child) -> impl WidgetBuilder {
    fn_widget! { child }
  }
}

#[derive(Declare)]
pub struct Router {
  cur_path: CowArc<str>,
}

impl ComposeChild for Router {
  type Child = Vec<Pair<State<Route>, Widget>>;
  fn compose_child(this: impl StateWriter<Value = Self>, child: Self::Child) -> impl WidgetBuilder {
    fn_widget! {
      @Stack {
        fit: StackFit::Expand,
        @ {
          child.into_iter().map(|p| {
            let (route, child) = p.unzip();
            let path = $route.path.clone();
            @Visibility {
              visible: pipe!(path.is_match($this.cur_path.as_ref())),
              @ { child }
            }
          })
        }
      }
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn partial_path() {
    let path_level_0 = PartialPath::new("/user", 0);
    let path_level_1 = PartialPath::new("/:id", 1);
    let path_level_2 = PartialPath::new("/info", 2);

    let path = "/user/123/info";

    assert!(path_level_0.is_match(path));
    assert_eq!(path_level_0.get_param(path), None);
    assert!(path_level_1.is_match(path));
    assert_eq!(path_level_1.get_param(path), Some("123".to_owned()));
    assert!(path_level_2.is_match(path));
    assert_eq!(path_level_2.get_param(path), None);
  }
}
