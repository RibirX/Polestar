use crate::style::WHITE;

use super::app::AppGUI;
use super::common::InteractList;
use ribir::prelude::*;

pub fn channel(_app: impl StateWriter<Value = AppGUI>) -> impl WidgetBuilder {
  fn_widget! {
    @InteractList {
      need_highlight: true,
      @PointerListener {
        on_tap: move |_| {
          println!("channel 1 tapped");
        },
        @ListItem {
          active_background: Color::from_u32(WHITE),
          @HeadlineText(Label::new("channel 1"))
        }
      }
      @PointerListener {
        on_tap: move |_| {
          println!("channel 2 tapped");
        },
        @ListItem {
          active_background: Color::from_u32(WHITE),
          @HeadlineText(Label::new("channel 2"))
        }
      }
      @PointerListener {
        on_tap: move |_| {
          println!("channel 3 tapped");
        },
        @ListItem {
          active_background: Color::from_u32(WHITE),
          @HeadlineText(Label::new("channel 3"))
        }
      }
    }
  }
}
