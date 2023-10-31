use super::app::AppGUI;
use super::common::{IconButton, InteractiveList};
use polestar_core::model::{Channel, BotAvatar};
use ribir::prelude::*;

pub fn channel(app: impl StateWriter<Value = AppGUI>) -> impl WidgetBuilder {
  fn_widget! {
    @InteractiveList {
      highlight_visible: true,
      @ {
        // TODO: here `channels` clone performance?
        $app.data.channels().clone().into_iter().map(|channel| {
          @PointerListener {
            on_tap: move |_| {

            },
            @ { ChannelView { data: channel } }
          }
        })
      }
    }
  }
}

#[derive(Declare)]
struct ChannelAvatar {
  avatar: BotAvatar,
}

impl Compose for ChannelAvatar {
  fn compose(this: impl StateWriter<Value = Self>) -> impl WidgetBuilder {
    fn_widget! {
      @ {
        match &$this.avatar {
          BotAvatar::Text { name, color } => {
            @Void {}
          },
          BotAvatar::Image { url } => {
            @Void {}
          },
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
            CustomEdgeWidget(@Void {}.widget_build(ctx!()))
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
                icon: ShareResource::new(include_svg!("../../static/send.svg")),
              }
              @IconButton {
                icon: ShareResource::new(include_svg!("../../static/send.svg")),
              }
            };
            CustomEdgeWidget(channel_operator.widget_build(ctx!()))
          }
        }
      }
    }
  }
}
