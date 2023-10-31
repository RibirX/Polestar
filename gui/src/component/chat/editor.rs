use ribir::prelude::*;

use crate::{component::common::IconButton, style::GRAY_F4};

pub fn editor() -> impl WidgetBuilder {
  fn_widget! {
    let editor_disabled = @IgnorePointer { ignore: false };
    let editor_textarea = @ { editor_textarea() };
    @$editor_disabled {
      @ConstrainedBox {
        clamp: BoxClamp {
          min: Size::new(f32::INFINITY, 56.),
          max: Size::new(f32::INFINITY, 104.),
        },
        padding: EdgeInsets::new(22., 11., 11., 11.),
        @Column {
          background: Color::from_u32(GRAY_F4),
          padding: EdgeInsets::all(10.),
          border_radius: Radius::all(8.),
          @ { editor_ref() }
          @ { editor_textarea }
        }
      }
    }
  }
}

fn editor_ref() -> impl WidgetBuilder {
  fn_widget! {
    @Void {}
  }
}

fn editor_textarea() -> impl WidgetBuilder {
  fn_widget! {
    let textarea = @TextArea {
      auto_wrap: true,
      cursor: CursorIcon::Text,
      padding: EdgeInsets::all(5.),
      @ { Placeholder::new("Type a message") }
    };

    @Row {
      @Expanded {
        flex: 1.,
        @ { textarea }
      }
      @IconButton {
        icon: ShareResource::new(include_svg!("../../../static/send.svg"))
      }
    }
  }
}
