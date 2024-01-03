use ribir::prelude::*;

use crate::{
  platform,
  style::{COMMON_RADIUS, WHITE},
  widgets::app::AppGUI,
};

mod account;
mod general;
mod network;
use account::{w_email, w_subscription, AccountItem};
use general::w_general_settings;
use network::w_network_settings;

pub fn w_settings(app: impl StateWriter<Value = AppGUI>) -> impl WidgetBuilder {
  fn_widget! {
    @ConstrainedBox {
      clamp: BoxClamp::EXPAND_BOTH,
      background: Color::from_u32(WHITE),
      border_radius: COMMON_RADIUS,
      @VScrollBar {
        h_align: HAlign::Center,
        @Column {
          margin: EdgeInsets::all(20.),
          @SettingItem {
            name: "Account",
            @AccountItem {
              name: "Email",
              @ { w_email(&app) }
            }
            @AccountItem {
              name: "Subscription",
              @ { w_subscription() }
            }
          }
          @ {
            (!platform::has_permission()).then(|| {
              @SettingItem {
                name: "General Settings",
                @ { w_general_settings() }
              }
            })
          }
          @SettingItem {
            name: "Network Settings",
            @ { w_network_settings() }
          }
        }
      }
    }
  }
}

#[derive(Declare)]
struct SettingItem {
  name: CowArc<str>,
}

impl ComposeChild for SettingItem {
  type Child = Vec<Widget>;

  fn compose_child(this: impl StateWriter<Value = Self>, child: Self::Child) -> impl WidgetBuilder {
    fn_widget! {
      @Column {
        @Text {
          text: $this.name.to_owned(),
          text_style: TypographyTheme::of(ctx!()).title_medium.text.clone(),
        }

        @ { child }
      }
    }
  }
}
