use ribir::prelude::*;

use super::app::AppGUI;

use crate::{
  component::common::{Carousel, DoubleColumn, LeftColumn, RightColumn},
  style::BLACK,
};

pub(super) fn w_permission(app: impl StateWriter<Value = AppGUI>) -> impl WidgetBuilder {
  fn_widget! {
    @DoubleColumn {
      fixed_width: 390.,
      @LeftColumn {
        @Column {
          margin: EdgeInsets::new(0., 30., 0., 40.),
          @ { w_intro() }
          @ { w_steps() }
        }
      }
      @RightColumn {
        @Carousel {
          contents: vec![]
        }
      }
    }
  }
}

fn w_intro() -> impl WidgetBuilder {
  fn_widget! {
    @Column {
      margin: EdgeInsets::only_bottom(40.),
      item_gap: 8.,
    }
  }
}

fn w_steps() -> impl WidgetBuilder {
  fn_widget! {
    @Column {
      item_gap: 30.,
      // step 1
      @Column {
        item_gap: 8.,
        @ { w_step_tip("Step 1") }
        @SizedBox {
          size: Size::new(300., 32.),
          @FilledButton {
            cursor: CursorIcon::Hand,
            color: Color::from_u32(BLACK),
            on_tap: move |_| {

            },
            @ { Label::new("Allow Permission") }
          }
        }
      }
      @ { w_step_tip("Step 2") }
      @ { w_step_tip("Step 3") }
      @Button {
        h_align: HAlign::Right,
        cursor: CursorIcon::Hand,
        color: Color::from_u32(BLACK),
        on_tap: move |_| {

        },
        @ { Label::new("Skip") }
      }
    }
  }
}

fn w_step_tip(text: impl Into<CowArc<str>>) -> impl WidgetBuilder {
  fn_widget! {
    @Text {
      text: text.into(),
      text_style: TypographyTheme::of(ctx!()).title_medium.text.clone(),
      overflow: Overflow::AutoWrap,
    }
  }
}
