use ribir::prelude::*;

use crate::{
  component::app::AppGUI,
  style::{ANTI_FLASH_WHITE, COMMON_RADIUS, LIGHT_SILVER_15, SPANISH_GRAY, WHITE},
};

pub fn w_bot_store(app: impl StateReader<Value = AppGUI>) -> impl WidgetBuilder {
  fn_widget! {
    @ConstrainedBox {
      clamp: BoxClamp::EXPAND_BOTH,
      background: Color::from_u32(WHITE),
      border_radius: COMMON_RADIUS,
      @Column {
        align_items: Align::Stretch,
        h_align: HAlign::Center,
        @VScrollBar {
          @ { w_bot_list(app.clone_reader()) }
        }
      }
    }
  }
}

const CATEGORY_LIST: [&str; 10] = [
  "Image",
  "Writing",
  "Language",
  "Legal",
  "Marketing",
  "Teacher",
  "Assistant",
  "Entertainment",
  "Coach",
  "Interviewer",
];

fn w_bot_list(app: impl StateReader<Value = AppGUI>) -> impl WidgetBuilder {
  fn_widget! {
    @Column {
      margin: EdgeInsets::all(14.),
      @ {
        CATEGORY_LIST.iter().map(|cat| {
          @Column {
            @Text {
              margin: EdgeInsets::new(24., 0., 14., 0.),
              text: cat.to_owned(),
              text_style: TypographyTheme::of(ctx!()).title_large.text.clone()
            }
            @Row {
              wrap: true,
              item_gap: 8.,
              line_gap: 8.,
              @ {
                $app.data.bots().iter().filter(|bot| bot.cat() == Some(cat)).map(|bot| {
                  @Clip {
                    @SizedBox {
                      size: Size::new(200., 110.),
                      cursor: CursorIcon::Hand,
                      padding: EdgeInsets::all(12.),
                      background: Color::from_u32(ANTI_FLASH_WHITE),
                      border: Border::all(BorderSide {
                        width: 1.,
                        color: Color::from_u32(LIGHT_SILVER_15).into(),
                      }),
                      border_radius: COMMON_RADIUS,
                      on_tap: move |_| {

                      },
                      @Column {
                        @Row {
                          align_items: Align::Center,
                          @Text {
                            text: bot.name().to_owned(),
                            overflow: Overflow::AutoWrap,
                            text_style: TypographyTheme::of(ctx!()).title_medium.text.clone(),
                          }
                        }
                        @ {
                          bot.desc().map(|s| s.to_owned()).map(|desc| {
                            @Text {
                              text: desc,
                              overflow: Overflow::AutoWrap,
                              margin: EdgeInsets::only_top(10.),
                              text_style: TypographyTheme::of(ctx!()).title_small.text.clone(),
                              foreground: Color::from_u32(SPANISH_GRAY),
                            }
                          })
                        }
                      }
                    }
                  }
                }).collect::<Vec<_>>()
              }
            }
          }
        }).collect::<Vec<_>>()
      }
    }
  }
}
