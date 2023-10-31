use crate::theme::polestar_svgs;

use super::app::AppGUI;
use super::common::{IconButton, InteractiveList};
use polestar_core::model::Channel;
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
struct ChannelView {
  data: Channel,
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
