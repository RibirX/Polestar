use ribir::prelude::*;

use super::icon_button::IconButton;
use crate::style::WHITE;

#[derive(Declare)]
pub struct Modal {
  title: CowArc<str>,
  size: Size,
  confirm_cb: Box<dyn Fn()>,
  cancel_cb: Box<dyn Fn()>,
}

impl ComposeChild for Modal {
  type Child = Widget;
  fn compose_child(this: impl StateWriter<Value = Self>, child: Self::Child) -> impl WidgetBuilder {
    fn_widget! {
      @Stack {
        fit: StackFit::Expand,
        background: Color::from_u32(0x1C1C1EFF).with_alpha(0.3),
        @SizedBox {
          size: $this.size,
          v_align: VAlign::Center,
          h_align: HAlign::Center,
          background: Color::from_u32(WHITE),
          border_radius: Radius::all(8.),
          @Column {
            margin: EdgeInsets::all(14.),
            @ConstrainedBox {
              clamp: BoxClamp::fixed_height(40.),
              @Row {
                justify_content: JustifyContent::SpaceBetween,
                @Text {
                  text: $this.title.to_owned(),
                  text_style: TypographyTheme::of(ctx!()).title_large.text.clone(),
                }
                @IconButton {
                  on_tap: move |_| {
                    ($this.cancel_cb)();
                  },
                  @ { svgs::CLOSE }
                }
              }
            }
            @ { child }
            @Row {
              item_gap: 10.,
              h_align: HAlign::Right,
              v_align: VAlign::Bottom,
              @OutlinedButton {
                cursor: CursorIcon::Hand,
                on_tap: move |_| {
                  ($this.cancel_cb)();
                },
                @ { Label::new("Cancel") }
              }
              @FilledButton {
                cursor: CursorIcon::Hand,
                on_tap: move |_| {
                  ($this.confirm_cb)();
                },
                @ { Label::new("Confirm") }
              }
            }
          }
        }
      }
    }
  }
}
