use crate::theme::polestar_svgs;

use super::app::AppGUI;
use super::common::{IconButton, InteractiveList, AvatarWidget};
use polestar_core::model::{Channel, BotAvatar};
use ribir::prelude::*;

pub fn channel(app: impl StateWriter<Value = AppGUI>) -> impl WidgetBuilder {
  fn_widget! {
    let def_bot_id = *($app.data.cfg().def_bot_id());
    @VScrollBar {
      @InteractiveList {
        highlight_visible: true,
        @ {
          // TODO: here `channels` clone performance?
          $app.data.channels().clone().into_iter().map(|channel| {
            let bot_id = &channel.cfg().def_bot_id().unwrap_or_else(|| &def_bot_id);
            let avatar = $app.data.bots().iter().find(|bot| bot.id() == *bot_id).unwrap().avatar().clone();
            @PointerListener {
              on_tap: move |_| {
  
              },
              @ { ChannelView { data: channel, avatar } }
            }
          })
        }
      }
    }
  }
}

#[derive(Declare)]
struct ChannelView {
  data: Channel,
  avatar: BotAvatar,
}

impl Compose for ChannelView {
  fn compose(this: impl StateWriter<Value = Self>) -> impl WidgetBuilder {
    fn_widget! {
      let mut item = @ListItem {};
      @$item {
        @Leading {
          @ {
            CustomEdgeWidget(
              @AvatarWidget {
                avatar: $this.avatar.clone(),
              }
              .widget_build(ctx!())
            )
          }
        }
        @HeadlineText(Label::new($this.data.name().to_owned()))
        @Trailing {
          @ {
            let channel_operator = @Row {
              visible: pipe!($item.mouse_hover()),
              align_items: Align::Center,
              item_gap: 8.,
              @IconButton {
                @ { polestar_svgs::EDIT }
              }
              @IconButton {
                @ { polestar_svgs::TRASH }
              }
            };
            CustomEdgeWidget(channel_operator.widget_build(ctx!()))
          }
        }
      }
    }
  }
}
