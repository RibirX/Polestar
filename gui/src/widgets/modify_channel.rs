use polestar_core::model::ChannelMode;
use ribir::prelude::*;
use uuid::Uuid;

use crate::style::{CHINESE_WHITE, COMMON_RADIUS, CULTURED_F7F7F5_FF, WHITE};
use crate::widgets::common::{w_avatar, BotList, Modal};

use super::app::AppGUI;

fn w_mode_options(
  channel: impl StateReader<Value = ChannelState>,
  channel_mode: ChannelMode,
) -> impl WidgetBuilder {
  fn_widget! {
    @ConstrainedBox {
      clamp: BoxClamp {
        min: Size::new(500., 40.),
        max: Size::new(500., 65.),
      },
      cursor: CursorIcon::Pointer,
      padding: EdgeInsets::all(10.),
      border: Border::all(BorderSide {
        width: 1.,
        color: Color::from_u32(CHINESE_WHITE).into(),
      }),
      border_radius: COMMON_RADIUS,
      @Row {
        align_items: Align::Center,
        @Checkbox {
          checked: pipe!($channel.channel_mode == channel_mode),
        }
        @Column {
          margin: EdgeInsets::only_left(6.),
          @Text {
            margin: EdgeInsets::only_bottom(4.),
            text: match channel_mode {
              ChannelMode::Balanced => "Balanced",
              ChannelMode::Performance => "Performance",
            },
            text_style: TypographyTheme::of(ctx!()).title_small.text.clone(),
          }
          @Text {
            text: "This mode automatically uses the advanced model and you will get more accurate answers but costs will increase.",
            overflow: Overflow::AutoWrap,
          }
        }
      }
    }
  }
}

#[derive(Default)]
struct ChannelState {
  pub bot_list_visible: bool,
  pub bot_list_top: f32,
  pub channel_mode: ChannelMode,
  pub selected_bot: Option<Uuid>,
}

pub fn w_modify_channel_modal(
  app: impl StateWriter<Value = AppGUI>,
  channel_id: &Uuid,
) -> impl WidgetBuilder {
  let channel_id = *channel_id;
  let channel = app.split_writer(
    move |app| {
      app
        .data
        .get_channel(&channel_id)
        .expect("channel not found")
    },
    move |app| {
      app
        .data
        .get_channel_mut(&channel_id)
        .expect("channel not found")
    },
  );

  fn_widget! {
    let channel_rename = @Input {};
    $channel_rename.write().set_text($channel.name());

    watch!($channel.name().to_owned())
      .distinct_until_changed()
      .subscribe(move |name| {
        $channel_rename.write().set_text(&name);
      });

    let app_writer_cancel_ref = app.clone_writer();
    let app_writer_confirm_ref = app.clone_writer();

    let bot_list = @BotList {
      bots: $app.data.info().bots_rc(),
    };

    let mut selected_bot_box = @LayoutBox {};

    let channel_state = State::value(ChannelState {
      channel_mode: $channel.cfg().mode(),
      ..Default::default()
    });

    let balanced_mode = w_mode_options(channel_state.clone_reader(), ChannelMode::Balanced);
    let performance_mode = w_mode_options(channel_state.clone_reader(), ChannelMode::Performance);

    watch!($bot_list.selected_bot())
      .distinct_until_changed()
      .subscribe(move |selected_bot| {
        if let Some((bot_id, _)) = selected_bot {
          $channel_state.write().selected_bot = Some(bot_id);
        } else {
          $channel_state.write().selected_bot = None;
        }
      });

    @Modal {
      title: "Channel Settings",
      size: Size::new(480., 530.),
      confirm_cb: Box::new(move || {
        let rename = $channel_rename;
        $channel.write().set_name(rename.text().to_string());

        if let Some(bot_id) = $channel_state.selected_bot {
          $channel.write().cfg_mut().set_def_bot_id(Some(bot_id));
        }

        if $channel_state.channel_mode != $channel.cfg().mode() {
          $channel.write().cfg_mut().set_mode($channel_state.channel_mode);
        }

        app_writer_confirm_ref.write().set_modify_channel_id(None);
      }) as Box<dyn Fn()>,
      cancel_cb: Box::new(move || {
        app_writer_cancel_ref.write().set_modify_channel_id(None);
      }) as Box<dyn Fn()>,
      @Stack {
        @Column {
          @Text {
            text: "Channel Name",
            text_style: TypographyTheme::of(ctx!()).title_medium.text.clone(),
          }
          @$channel_rename {
            cursor: CursorIcon::Text,
            h_align: HAlign::Stretch,
            margin: EdgeInsets::only_bottom(10.),
            background: Color::from_u32(CULTURED_F7F7F5_FF),
            padding: EdgeInsets::new(10., 5., 10., 5.),
            border: Border::all(BorderSide {
              width: 1.,
              color: Color::from_u32(CHINESE_WHITE).into(),
            }),
            border_radius: Radius::all(6.),
            @ { Placeholder::new("Type a channel name") }
          }
          @Text {
            text: "Default AI Bot",
            text_style: TypographyTheme::of(ctx!()).title_medium.text.clone(),
          }
          @$selected_bot_box {
            @ {
              pipe! {
                let app_ref = $app;
                let bot = app_ref.data.info().get_bot_or_default($channel_state.selected_bot);
                @ListItem {
                  on_tap: move |e| {
                    if !$channel_state.bot_list_visible {
                      let pos = Point::new(0., $selected_bot_box.layout_height());
                      $channel_state.write().bot_list_visible = true;
                      $channel_state.write().bot_list_top = e.map_to_parent(pos).y;
                    } else {
                      $channel_state.write().bot_list_visible = false;
                    }
                  },
                  @Leading {
                    @ {
                      CustomEdgeWidget(
                        w_avatar(bot.avatar().clone()).widget_build(ctx!())
                      )
                    }
                  }
                  @HeadlineText(Label::new(bot.name().to_owned()))
                }
              }
            }
          }
          @Text {
            text: "Optimize Performance",
            text_style: TypographyTheme::of(ctx!()).title_medium.text.clone(),
          }
          @Column {
            @$balanced_mode {
              on_tap: move |_| {
                $channel_state.write().channel_mode = ChannelMode::Balanced;
              }
            }
            @$performance_mode {
              on_tap: move |_| {
                $channel_state.write().channel_mode = ChannelMode::Performance;
              }
            }
          }
        }
        @ConstrainedBox {
          clamp: BoxClamp::fixed_height(120.),
          background: Color::from_u32(WHITE),
          visible: pipe!($channel_state.bot_list_visible),
          anchor: pipe!(Anchor::top($channel_state.bot_list_top)),
          on_tap: move |_| {
            $channel_state.write().bot_list_visible = false;
          },
          @ { bot_list }
        }
      }
    }
  }
}
