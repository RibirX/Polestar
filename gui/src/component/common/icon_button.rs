use ribir::prelude::*;

#[derive(Declare)]
pub struct IconButton {
  icon: ShareResource<Svg>,
}

impl Compose for IconButton {
  fn compose(this: impl StateWriter<Value = Self>) -> impl WidgetBuilder {
    fn_widget! {
      @Icon {
        cursor: CursorIcon::Hand,
        size: Size::splat(24.),
        @ { $this.icon.clone() }
      }
    }
  }
}
