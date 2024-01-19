use ribir::prelude::*;

use crate::style::CULTURED_F7F7F5_FF;

#[derive(PartialEq, Eq)]
pub enum FixedSide {
  Left,
  // Right,
}

#[derive(Declare)]
pub struct DoubleColumn {
  #[declare(default = FixedSide::Left)]
  fixed_side: FixedSide,
  fixed_width: f32,
}

#[derive(Declare, PairChild)]
pub struct LeftColumn;

#[derive(Declare, PairChild)]
pub struct RightColumn;

#[derive(Template)]
pub struct DoubleColumnTpl {
  left: WidgetOf<LeftColumn>,
  right: WidgetOf<RightColumn>,
}

impl ComposeChild for DoubleColumn {
  type Child = DoubleColumnTpl;

  fn compose_child(this: impl StateWriter<Value = Self>, child: Self::Child) -> impl WidgetBuilder {
    fn_widget! {
      @ConstrainedBox {
        clamp: BoxClamp::EXPAND_BOTH,
        background: Color::from_u32(CULTURED_F7F7F5_FF),
        @ {
          if $this.fixed_side == FixedSide::Left {
            @Row {
              @ConstrainedBox {
                clamp: BoxClamp::fixed_width($this.fixed_width),
                @ { child.left.child() }
              }
              @Expanded {
                flex: 1.,
                @ { child.right.child() }
              }
            }
          } else {
            @Row {
              @Expanded {
                flex: 1.,
                @ { child.left.child() }
              }
              @ConstrainedBox {
                clamp: BoxClamp::fixed_width($this.fixed_width),
                @HAlignWidget {
                  h_align: HAlign::Center,
                  @ { child.right.child() }
                }
              }
            }
          }
        }
      }
    }
  }
}
