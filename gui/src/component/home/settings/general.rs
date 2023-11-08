use ribir::prelude::*;

use crate::style::BLACK;

pub(super) fn w_general_settings() -> impl WidgetBuilder {
  fn_widget! {
    @Row {
      justify_content: JustifyContent::SpaceBetween,
      @ConstrainedBox {
        clamp: BoxClamp::fixed_width(500.),
        @Column {
          @Text {
            text: "Quick Launcher Accessibility Access Permission",
            text_style: TypographyTheme::of(ctx!()).title_small.text.clone(),
          }
          @Text {
            text: "If you are unable to use Quick Launcher, you may need to grant accessibility access permission to PoleStar",
            foreground: Palette::of(ctx!()).outline(),
            overflow: Overflow::AutoWrap,
          }
        }
      }
      @FilledButton {
        cursor: CursorIcon::Hand,
        color: Color::from_u32(BLACK),
        @ { Label::new("Allow Permission") }
      }
    }
  }
}
