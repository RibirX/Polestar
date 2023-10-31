use polestar_core::model::{Channel, MsgRole};
use ribir::prelude::*;
use uuid::Uuid;

use crate::style::{self, BOT_MESSAGE_BACKGROUND, GRAY_HIGHLIGHT_BORDER, USER_MESSAGE_BACKGROUND};

pub fn chat_list() -> impl WidgetBuilder {
  fn_widget! {
    let scrollable_container = @VScrollBar {};

    @ConstrainedBox {
      clamp: BoxClamp::EXPAND_BOTH,
      @$scrollable_container {
        @ConstrainedBox {
          clamp: BoxClamp {
            min: Size::new(500., 0.),
            max: Size::new(800., f32::INFINITY),
          },
          @Column {
            padding: EdgeInsets::all(16.),
            item_gap: 16.,
            @ { chat_onboarding() }
            @ { content() }
          }
        }
      }
    }
  }
}

fn chat_onboarding() -> impl WidgetBuilder {
  fn_widget! {
    @Void {}
  }
}

fn content() -> impl WidgetBuilder {
  fn_widget! {
    @Column {
      item_gap: 16.,
      // User message
      @Row {
        h_align: HAlign::Right,
        @Stack {
          @ConstrainedBox {
            clamp: BoxClamp {
              min: Size::zero(),
              max: Size::new(560., f32::INFINITY),
            },
            @ {
              style::decorator::channel::message_style( fn_widget! {
                @Column {
                  @TextSelectable {
                    @Text {
                      text: "hello world",
                      text_style: TypographyTheme::of(ctx!()).body_large.text.clone()
                    }
                  }
                }
              }, MsgRole::User)
            }
          }
        }
      }
      // Bot message
      @Row {
        h_align: HAlign::Left,
        @Stack {
          @ConstrainedBox {
            clamp: BoxClamp {
              min: Size::zero(),
              max: Size::new(560., f32::INFINITY),
            },
            @ {
              style::decorator::channel::message_style( fn_widget! {
                @Column {
                  @TextSelectable {
                    @Text {
                      text: "hello world",
                      text_style: TypographyTheme::of(ctx!()).body_large.text.clone()
                    }
                  }
                }
              }, MsgRole::Bot(Uuid::nil()))
            }
          }
        }
      }
    }
  }
}
