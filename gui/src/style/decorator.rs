pub mod common {}

pub mod channel {
  use polestar_core::model::MsgRole;
  use ribir::prelude::*;

  use crate::style::{
    BOT_MESSAGE_BACKGROUND, GRAY_HIGHLIGHT, GRAY_HIGHLIGHT_BORDER, USER_MESSAGE_BACKGROUND,
  };

  pub fn highlight_style(host: Widget) -> impl WidgetBuilder {
    fn_widget! {
      @ $host {
        background: Color::from_u32(GRAY_HIGHLIGHT),
        border_radius: Radius::all(8.),
        border: Border::all(BorderSide {
          color: Color::from_u32(GRAY_HIGHLIGHT_BORDER).into(),
          width: 1.,
        }),
      }
    }
  }

  pub fn message_style(host: impl WidgetBuilder, role: MsgRole) -> impl WidgetBuilder {
    fn_widget! {
      @ $host {
        padding: EdgeInsets::new(10., 14., 12., 14.),
        border_radius: Radius::all(8.),
        border: Border::all(BorderSide {
          color: Color::from_u32(GRAY_HIGHLIGHT_BORDER).into(),
          width: 1.,
        }),
        background: {
          match role {
            MsgRole::Bot(_) => Color::from_u32(BOT_MESSAGE_BACKGROUND),
            MsgRole::User => Color::from_u32(USER_MESSAGE_BACKGROUND),
          }
        },
      }
    }
  }
}
