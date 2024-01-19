use polestar_core::{model::ChannelCfg, open_user_config_folder};
use ribir::prelude::*;

use crate::{
  style::APP_SIDEBAR_HEADER_HEIGHT,
  widgets::{
    app::{ChannelMgr, UIState},
    common::{IconButton, InteractiveList},
  },
  G_APP_NAME,
};

mod channel_thumbnail_list;
use channel_thumbnail_list::w_channel_thumbnail_list;

pub fn w_sidebar(
  channel_mgr: impl StateWriter<Value = dyn ChannelMgr>,
  ui_state: impl StateWriter<Value = dyn UIState>,
) -> impl WidgetBuilder {
  fn_widget! {
    @Column {
      @ { w_sidebar_header(channel_mgr.clone_writer()) }
      @Expanded {
        flex: 1.,
        @ { w_channel_thumbnail_list(channel_mgr.clone_writer(), ui_state.clone_writer()) }
      }
      @ { w_sidebar_others(channel_mgr.clone_writer(), ui_state) }
    }
  }
}

fn w_sidebar_header(chat_mgr: impl StateWriter<Value = dyn ChannelMgr>) -> impl WidgetBuilder {
  fn_widget! {
    @ConstrainedBox {
      clamp: BoxClamp::fixed_height(APP_SIDEBAR_HEADER_HEIGHT),
      @Row {
        padding: EdgeInsets::new(10., 5., 5., 15.),
        justify_content: JustifyContent::SpaceBetween,
        @Text {
          on_tap: move |_| {
            open_user_config_folder();
          },
          margin: EdgeInsets::only_left(6.),
          text: G_APP_NAME,
          text_style: TypographyTheme::of(ctx!()).title_large.text.clone()
        }
        @IconButton {
          size: IconSize::of(ctx!()).medium,
          on_tap: move |_| {
            let channel_id = $chat_mgr.write().new_channel("Untitled".to_owned(), None, ChannelCfg::default());
            let chat_mgr = chat_mgr.clone_writer();
            let _ = AppCtx::spawn_local(async move {
              $chat_mgr.write().switch_channel(&channel_id);
            });
          },
          @ { svgs::ADD }
        }
      }
    }
  }
}

fn w_sidebar_others(
  chat_mgr: impl StateWriter<Value = dyn ChannelMgr>,
  ui_state: impl StateWriter<Value = dyn UIState>,
) -> impl WidgetBuilder {
  fn_widget! {
    @InteractiveList {
      highlight_visible: pipe! {
        let path = $ui_state.cur_path();
        matches!(path.as_str(), "/home/bot_store" | "/home/settings")
      },
      @ListItem {
        on_tap: move |_| {
          $ui_state.write().navigate_to("/home/bot_store");
        },
        @HeadlineText(Label::new("BotStore"))
      }
      @ListItem {
        on_tap: move |_| {
          $ui_state.write().navigate_to("/home/settings");
        },
        @HeadlineText(Label::new("Setting"))
      }
      @ListItem {
        on_tap: move |_| {
          let ids = $chat_mgr.channel_ids();
          let feedback_id = ids.iter().find(|channel_id| {
            $chat_mgr.channel(channel_id).unwrap().is_feedback()
          });

          if let Some(feedback_id) = feedback_id {
            $chat_mgr.write().switch_channel(feedback_id);
          } else {
            let id = $chat_mgr.write().new_channel("Feedback".to_owned(), None, ChannelCfg::feedback_cfg());
            $chat_mgr.write().switch_channel(&id);
          }
          $ui_state.write().navigate_to("/home/chat");
        },
        @HeadlineText(Label::new("Feedback"))
      }
    }
  }
}
