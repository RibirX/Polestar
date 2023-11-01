use ribir::prelude::*;

use super::icon_button::IconButton;
use crate::style::WHITE;

#[derive(Declare)]
pub struct Modal;

impl ComposeChild for Modal {
  type Child = Widget;
  fn compose_child(_: impl StateWriter<Value = Self>, child: Self::Child) -> impl WidgetBuilder {
    fn_widget! {
      @Stack {
        fit: StackFit::Passthrough,
        background: Color::from_u32(0x1C1C1EFF).with_alpha(0.3),
        @Column {
          v_align: VAlign::Center,
          h_align: HAlign::Center,
          background: Color::from_u32(WHITE),
          border_radius: Radius::all(8.),
          @IconButton {
            @ { svgs::CLOSE }
          }
          @ { child }
          @Row {
            h_align: HAlign::Right,
            v_align: VAlign::Bottom,
            @OutlinedButton {
              @ { Label::new("Cancel") }
            }
            @FilledButton {
              @ { Label::new("Confirm") }
            }
          }
        }
      }
    }
  }
}
