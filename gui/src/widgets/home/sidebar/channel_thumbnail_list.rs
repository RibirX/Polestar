use crate::theme::polestar_svg;

use crate::widgets::{
  app::AppGUI,
  common::{w_avatar, IconButton, InteractiveList},
};
use polestar_core::model::Channel;
use ribir::prelude::*;

pub fn w_channel_thumbnail_list(app: impl StateWriter<Value = AppGUI>) -> impl WidgetBuilder {
  fn_widget! {
    @InteractiveList {
      active: pipe! {
        let last_idx = if !$app.data.channels().is_empty() { 
          $app.data.channels().len() - 1
        } else {
          0
        };
        $app.data.cur_channel().map(|channel| *channel.id()).and_then(|id| {
          $app.data.channels().iter().position(|channel| *channel.id() == id).map(|idx| last_idx - idx)
        }).unwrap_or(last_idx)
      },
      on_tap: move |_| {
        if $app.cur_router_path() != "/home/chat" {
          $app.write().navigate_to("/home/chat");
        }
      },
      highlight_visible: pipe! {
        let app = $app;
        matches!(app.cur_router_path(), "/home/chat")
      },
      @ {
        pipe! {
          let app2 = app.clone_writer();
          let mut rst = vec![];
          for idx in (0..$app.data.channels().len()).rev() {
            let channel = app2.split_writer(
              move |app| { app.data.channels().iter().nth(idx).expect("channel must be existed") },
              move |app| { app.data.channels_mut().iter_mut().nth(idx).expect("channel must be existed") },
            );
            let channel2 = channel.clone_writer();

            let w_channel_thumbnail = w_channel_thumbnail(channel);
            let w_channel_thumbnail = @ $w_channel_thumbnail {
              on_tap: move |_| {
                let id = *$channel2.id();
                $app.write().data.switch_channel(&id);
              },
            };
            rst.push(w_channel_thumbnail);
          }
          rst
        }
      }
    }
  }
}

fn w_channel_thumbnail<S>(channel: S) -> impl WidgetBuilder
where
  S: StateWriter<Value = Channel> + 'static,
  S::OriginWriter: StateWriter<Value = AppGUI>,
{
  let app = channel.origin_writer().clone_writer();
  fn_widget! {
    let mut item = @ListItem {};
    // let support_text = $channel.desc().map(|desc| {
    //   @SupportingText(Label::new(desc.to_owned()))
    // });

    @$item {
      @Leading {
        @ {
          let channel_state = $channel;
          let app_state = $app;
          let channel_def_bot_id = channel_state.cfg().def_bot_id();
          let bot_id = *(channel_def_bot_id.unwrap_or_else(|| app_state.data.info().cfg().def_bot_id()));
          let avatar = app_state
            .data
            .info()
            .bots()
            .iter()
            .find(|bot| *bot.id() == bot_id)
            .expect("bot must be exist")
            .avatar();
          CustomEdgeWidget(
            w_avatar(avatar.clone()).widget_build(ctx!())
          )
        }
      }
      @HeadlineText(Label::new($channel.name().to_owned()))
      // TODO: add support text
      @Trailing {
        @ {
          CustomEdgeWidget(@Row {
            visible: pipe!($item.mouse_hover()),
            align_items: Align::Center,
            item_gap: 8.,
            @IconButton {
              on_tap: move |e| {
                let id = *$channel.id();
                $app.write().set_modify_channel_id(Some(id));
                e.stop_propagation();
              },
              @ { polestar_svg::EDIT }
            }
            @IconButton {
              on_tap: move |e| {
                let id = *$channel.id();
                $app.write().data.remove_channel(&id);
                e.stop_propagation();
              },
              @ { polestar_svg::TRASH }
            }
          }.widget_build(ctx!()))
        }
      }
    }
  }
}
