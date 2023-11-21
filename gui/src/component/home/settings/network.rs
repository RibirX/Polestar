use ribir::prelude::*;

use crate::style::{BRIGHT_GRAY_EAE9E9_FF, CHINESE_WHITE, CULTURED_F7F7F5_FF};

pub(super) fn w_network_settings() -> impl WidgetBuilder {
  fn_widget! {
    @Column {
      @Text {
        text: "Network Settings",
        text_style: TypographyTheme::of(ctx!()).title_medium.text.clone(),
      }
      @Row {
        padding: EdgeInsets::only_bottom(5.),
        @Text {
          text: "If you can not get the message, please fill in the proxy address here.",
          foreground: Palette::of(ctx!()).outline(),
        }
        @Row {
          @Text {
            text: "Or you can check out the details",
            foreground: Palette::of(ctx!()).outline(),
          }
          @Link {
            url: "https://statuesque-goal-eef.notion.site/103-How-to-use-de5e8f1687fb4afa9fcd1a6825eca5a2",
            cursor: CursorIcon::Pointer,
            @Text {
              text: "here",
              foreground: Palette::of(ctx!()).primary(),
            }
          }
        }
      }
      @ { w_setting_input() }
    }
  }
}

fn w_setting_input() -> impl WidgetBuilder {
  fn_widget! {
    let input = @Input {
      cursor: CursorIcon::Text,
      background: Color::from_u32(CULTURED_F7F7F5_FF),
      padding: EdgeInsets::new(10., 5., 10., 5.),
      border: Border::all(BorderSide {
        width: 1.,
        color: Color::from_u32(CHINESE_WHITE).into(),
      }),
      border_radius: Radius::all(6.),
      @ { Placeholder::new("Input your proxy address here") }
    };

    @Row {
      justify_content: JustifyContent::SpaceBetween,
      align_items: Align::Center,
      @Expanded {
        flex: 1.,
        @ { input }
      }
      @Button {
        cursor: CursorIcon::Pointer,
        color: Color::from_u32(BRIGHT_GRAY_EAE9E9_FF),
        margin: EdgeInsets::only_left(10.),
        on_tap: move |_| {

        },
        @ { Label::new("Save") }
      }
    }
  }
}
