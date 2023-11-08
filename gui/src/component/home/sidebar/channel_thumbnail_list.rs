use crate::theme::polestar_svg;

use crate::component::{
  app::AppGUI,
  common::{w_avatar, IconButton, InteractiveList},
};
use polestar_core::model::Channel;
use ribir::prelude::*;

pub fn w_channel_thumbnail_list(app: impl StateWriter<Value = AppGUI>) -> impl WidgetBuilder {
  fn_widget! {
    @VScrollBar {
      @InteractiveList {
        highlight_visible: true,
        @ {
          let app2 = app.clone_writer();
          $app.data.channels().iter().enumerate().map(move |(idx, _)| {
            let channel = app2.split_writer(
              move |app| { app.data.channels().get(idx).expect("channel must be existed") },
              move |app| { app.data.channels_mut().get_mut(idx).expect("channel must be existed") },
            );
            let channel2 = channel.clone_writer();

            @PointerListener {
              on_tap: move |_| {
                println!("channel: {}", channel2.read().name());
              },
              @ { w_channel_thumbnail(channel) }
            }
          }).collect::<Vec<_>>()
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
    let support_text = $channel.desc().map(|desc| {
      @SupportingText(Label::new(desc.to_owned()))
    });

    @$item {
      @Leading {
        @ {
          let channel_state = $channel;
          let app_state = $app;
          let channel_def_bot_id = channel_state.cfg().def_bot_id();
          let bot_id = *(channel_def_bot_id.unwrap_or_else(|| app_state.data.cfg().def_bot_id()));
          let avatar = app.map_reader(move |app| {
            app.data.bots().iter().find(|bot| *bot.id() == bot_id).expect("bot must be exist").avatar()
          });
          CustomEdgeWidget(
            w_avatar(&avatar).widget_build(ctx!())
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
              on_tap: move |_| {
                let id = *$channel.id();
                $app.write().set_modify_channel_id(Some(id));
              },
              @ { polestar_svg::EDIT }
            }
            @IconButton {
              @ { polestar_svg::TRASH }
            }
          }.widget_build(ctx!()))
        }
      }
    }
  }
}
