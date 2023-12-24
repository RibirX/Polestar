use polestar_core::model::{ChannelCfg, MsgMeta, Msg};
use ribir::prelude::*;

use crate::{
  style::{ANTI_FLASH_WHITE, COMMON_RADIUS, LIGHT_SILVER_15, SPANISH_GRAY, WHITE},
  widgets::{app::AppGUI, helper::send_msg},
};

pub fn w_bot_store(app: impl StateWriter<Value = AppGUI>) -> impl WidgetBuilder {
  fn_widget! {
    @ConstrainedBox {
      clamp: BoxClamp::EXPAND_BOTH,
      background: Color::from_u32(WHITE),
      border_radius: COMMON_RADIUS,
      @Column {
        align_items: Align::Stretch,
        h_align: HAlign::Center,
        @VScrollBar {
          @ { w_bot_list(app.clone_writer()) }
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

fn w_bot_list(app: impl StateWriter<Value = AppGUI>) -> impl WidgetBuilder {
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
                $app.data.info().bots().iter().filter(|bot| bot.cat() == Some(cat)).map(move |bot| {
                  let bot_name = bot.name().to_owned();
                  let bot_id = *bot.id();
                  let bot_onboarding = bot.onboarding().map_or(format!("@{bot_name}"), |str| {
                    format!("@{bot_name} {str}")
                  });
                  @Clip {
                    on_tap: move |_| {
                      let (channel_id, bot_msg_id) = {
                        let mut app_write = $app.write();
                        let channel_id = app_write.data.new_channel(bot_name.clone(), None, ChannelCfg::def_bot_id_cfg(bot_id));
                        let channel = app_write.data.get_channel_mut(&channel_id).unwrap();
                        let msg = Msg::new_user_text(&bot_onboarding, MsgMeta::default());
                        let user_msg_id = *msg.id();
                        channel.add_msg(msg);
                        let bot_msg = Msg::new_bot_text(bot_id, MsgMeta::reply(user_msg_id));
                        let bot_msg_id = *bot_msg.id();
                        channel.add_msg(bot_msg);
                        app_write.data.switch_channel(&channel_id);
                        app_write.navigate_to("/home/chat");
                        (channel_id, bot_msg_id)
                      };

                      println!("channel_id: {:?}, bot_msg_id: {:?}", channel_id, bot_msg_id);

                      let channel_writer = app.split_writer(
                        move |app| { app.data.get_channel(&channel_id).unwrap() },
                        move |app| { app.data.get_channel_mut(&channel_id).unwrap() },
                      );

                      send_msg(channel_writer, bot_onboarding.clone(), 0, bot_msg_id, bot_id);
                    },
                    @SizedBox {
                      size: Size::new(200., 110.),
                      cursor: CursorIcon::Pointer,
                      padding: EdgeInsets::all(12.),
                      background: Color::from_u32(ANTI_FLASH_WHITE),
                      border: Border::all(BorderSide {
                        width: 1.,
                        color: Color::from_u32(LIGHT_SILVER_15).into(),
                      }),
                      border_radius: COMMON_RADIUS,
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
