use ribir::prelude::*;

#[derive(Declare)]
pub struct IconButton;

impl ComposeChild for IconButton {
  type Child = Widget;
  fn compose_child(_: impl StateWriter<Value = Self>, child: Self::Child) -> impl WidgetBuilder {
    fn_widget! {
      @Icon {
        cursor: CursorIcon::Hand,
        size: Size::splat(24.),
        @ { child }
      }
    }
  }
}
