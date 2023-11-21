pub mod common {}

pub mod channel {
  use polestar_core::model::MsgRole;
  use ribir::prelude::*;

  use crate::style::{ALICE_BLUE, BRIGHT_GRAY_E9EEF7_FF, BRIGHT_GRAY_EDEDEB_FF, LIGHT_SILVER_15};

  pub fn highlight_style(host: Widget) -> impl WidgetBuilder {
    fn_widget! {
      @ $host {
        background: Color::from_u32(BRIGHT_GRAY_EDEDEB_FF),
        border_radius: Radius::all(8.),
        border: Border::all(BorderSide {
          color: Color::from_u32(LIGHT_SILVER_15).into(),
          width: 1.,
        }),
      }
    }
  }

  pub fn message_style(host: Widget, role: MsgRole) -> impl WidgetBuilder {
    fn_widget! {
      @ $host {
        padding: EdgeInsets::new(10., 14., 12., 14.),
        border_radius: Radius::all(8.),
        border: Border::all(BorderSide {
          color: Color::from_u32(LIGHT_SILVER_15).into(),
          width: 1.,
        }),
        background: {
          match role {
            MsgRole::Bot(_) => Color::from_u32(ALICE_BLUE),
            MsgRole::User => Color::from_u32(BRIGHT_GRAY_E9EEF7_FF),
          }
        },
      }
    }
  }
}
