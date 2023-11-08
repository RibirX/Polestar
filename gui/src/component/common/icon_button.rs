use ribir::{material::InteractiveLayer, prelude::*};

use crate::style::DARK_CHARCOAL;


#[derive(Declare)]
pub struct IconButton {
  #[declare(default = IconSize::of(ctx!()).small)]
  size: Size,
}

impl ComposeChild for IconButton {
  type Child = Widget;
  fn compose_child(this: impl StateWriter<Value = Self>, child: Self::Child) -> impl WidgetBuilder {
    fn_widget! {
      @InteractiveLayer {
        color: Color::from_u32(DARK_CHARCOAL),
        border_radii: Radius::all(4.),
        cursor: CursorIcon::Hand,
        @Icon {
          size: pipe!($this.size),
          @ { child }
        }
      }
    }
  }
}
