use ribir::prelude::*;

use crate::{
  style::{APP_SIDEBAR_HEADER_HEIGHT, GRAY},
  APP_NAME,
};

use super::app::AppGUI;

pub fn sidebar(app: impl StateWriter<Value = AppGUI>) -> impl WidgetBuilder {
  fn_widget! {
    @Column {
      @ { header(app.clone_writer()) }
    }
  }
}

fn header(app: impl StateWriter<Value = AppGUI>) -> impl WidgetBuilder {
  fn_widget! {
    @ConstrainedBox {
      clamp: BoxClamp::fixed_height(APP_SIDEBAR_HEADER_HEIGHT),
      @Row {
        padding: EdgeInsets::new(10., 5., 5., 15.),
        justify_content: JustifyContent::SpaceBetween,
        @Text {
          margin: EdgeInsets::only_left(6.),
          text: APP_NAME,
          text_style: TypographyTheme::of(ctx!()).title_large.text.clone()
        }
        @Button {
          cursor: CursorIcon::Hand,
          color: Color::from_u32(GRAY),
          @ { svgs::ADD }
        }
      }
    }
  }
}
