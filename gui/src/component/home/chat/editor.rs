use polestar_core::model::Channel;
use ribir::prelude::*;

use crate::{component::common::IconButton, style::CULTURED_F4F4F4_FF, theme::polestar_svg};

pub fn w_editor(channel: impl StateWriter<Value = Channel>) -> impl WidgetBuilder {
  fn_widget! {
    let ignore_pointer = @IgnorePointer { ignore: false };
    let textarea = @ { w_textarea() };
    @$ignore_pointer {
      @ConstrainedBox {
        clamp: BoxClamp {
          min: Size::new(f32::INFINITY, 56.),
          max: Size::new(f32::INFINITY, 104.),
        },
        padding: EdgeInsets::new(22., 11., 11., 11.),
        @Column {
          background: Color::from_u32(CULTURED_F4F4F4_FF),
          padding: EdgeInsets::all(10.),
          border_radius: Radius::all(8.),
          @ { w_editor_msg_quote() }
          @ { textarea }
        }
      }
    }
  }
}

fn w_editor_msg_quote() -> impl WidgetBuilder {
  fn_widget! {
    @Void {}
  }
}

fn w_textarea() -> impl WidgetBuilder {
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
        @ { polestar_svg::SEND }
      }
    }
  }
}
