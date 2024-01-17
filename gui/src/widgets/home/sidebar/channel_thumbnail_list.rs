use crate::theme::polestar_svg;

use crate::widgets::app::{UIState, ChannelMgr};
use crate::widgets::common::{w_avatar, IconButton, InteractiveList};
use polestar_core::model::Channel;
use ribir::prelude::*;

pub fn w_channel_thumbnail_list(
  channel_mgr: impl StateWriter<Value = dyn ChannelMgr>, 
  ui_state: impl StateWriter<Value = dyn UIState>) -> impl WidgetBuilder {
  fn_widget! {
    @InteractiveList {
      active: pipe! {
        let channels = $channel_mgr.channel_ids();
        let last_idx = if !channels.is_empty() { 
          channels.len() - 1
        } else {
          0
        };
        $channel_mgr.cur_channel_id().and_then(|id| {
          channels.iter().position(|ch| ch == id).map(|idx| last_idx - idx)
        }).unwrap_or(last_idx)
      },
      on_tap: move |_| {
        if $ui_state.cur_path() != "/home/chat" {
          $ui_state.write().navigate_to("/home/chat");
        }
      },
      highlight_visible: pipe! {
        let path = $ui_state.cur_path();
        matches!(path.as_str(), "/home/chat")
      },
      @ {
        pipe! {
          let mut rst = vec![];
          for id in $channel_mgr.channel_ids().into_iter().rev() {
            let channel = channel_mgr.map_reader(
              move |channels| { channels.channel(&id).expect("channel must be existed") },
            );
            let w_channel_thumbnail = w_channel_thumbnail(channel, channel_mgr.clone_writer(), ui_state.clone_writer());
            let w_channel_thumbnail = @ $w_channel_thumbnail {
              on_tap: move |_| {
                let _ = || $channel_mgr.write();
                let channel_mgr = channel_mgr.clone_writer();
                let _ = AppCtx::spawn_local(async move {
                  channel_mgr.write().switch_channel(&id);
                });
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

fn w_channel_thumbnail(channel: impl StateReader<Value = Channel>, channel_mgr: impl StateWriter<Value = dyn ChannelMgr>, gui_helper: impl StateWriter<Value = dyn UIState>) -> impl WidgetBuilder
{
  fn_widget! {
    let mut item = @ListItem {};
    // let support_text = $channel.desc().map(|desc| {
    //   @SupportingText(Label::new(desc.to_owned()))
    // });

    @$item {
      @Leading {
        @ {
          let channel_state = $channel;
          let channel_def_bot_id = channel_state.cfg().def_bot_id().cloned();
          let bot_id = channel_def_bot_id.unwrap_or_else(|| $channel.app_info().unwrap().cfg().def_bot_id().clone());
          let avatar = channel_state
            .app_info()
            .unwrap()
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
                $gui_helper.write().set_modify_channel_id(Some(id));
                e.stop_propagation();
              },
              @ { polestar_svg::EDIT }
            }
            @IconButton {
              on_tap: move |e| {
                let id = *$channel.id();
                $channel_mgr.write().remove_channel(&id);
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
