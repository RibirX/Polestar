use ribir::prelude::*;

use crate::{
  component::{
    app::AppGUI,
    common::{IconButton, InteractiveList},
  },
  style::APP_SIDEBAR_HEADER_HEIGHT,
  G_APP_NAME,
};

mod channel_thumbnail_list;
use channel_thumbnail_list::w_channel_thumbnail_list;

pub fn w_sidebar(app: impl StateWriter<Value = AppGUI>) -> impl WidgetBuilder {
  fn_widget! {
    @Column {
      @ { w_sidebar_header(app.clone_writer()) }
      @Expanded {
        flex: 1.,
        @ { w_channel_thumbnail_list(app.clone_writer()) }
      }
      @ { w_sidebar_others(app.clone_writer()) }
    }
  }
}

fn w_sidebar_header(app: impl StateWriter<Value = AppGUI>) -> impl WidgetBuilder {
  fn_widget! {
    @ConstrainedBox {
      clamp: BoxClamp::fixed_height(APP_SIDEBAR_HEADER_HEIGHT),
      @Row {
        padding: EdgeInsets::new(10., 5., 5., 15.),
        justify_content: JustifyContent::SpaceBetween,
        @Text {
          margin: EdgeInsets::only_left(6.),
          text: G_APP_NAME,
          text_style: TypographyTheme::of(ctx!()).title_large.text.clone()
        }
        @IconButton {
          size: IconSize::of(ctx!()).medium,
          on_tap: move |_| {
            $app.write().data.new_channel("Untitled".to_owned(), None);
          },
          @ { svgs::ADD }
        }
      }
    }
  }
}

fn w_sidebar_others(app: impl StateWriter<Value = AppGUI>) -> impl WidgetBuilder {
  fn_widget! {
    @InteractiveList {
      highlight_visible: pipe! {
        match $app.cur_router_path() {
          "/home/bot_store" => true,
          "/home/settings" => true,
          _ => false,
        }
      },
      @ListItem {
        on_tap: move |_| {
          $app.write().navigate_to("/home/bot_store");
        },
        @HeadlineText(Label::new("BotStore"))
      }
      @ListItem {
        on_tap: move |_| {
          $app.write().navigate_to("/home/settings");
        },
        @HeadlineText(Label::new("Setting"))
      }
      @ListItem {
        on_tap: move |_| {

        },
        @HeadlineText(Label::new("Feedback"))
      }
    }
  }
}
