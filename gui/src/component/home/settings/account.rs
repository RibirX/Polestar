use ribir::prelude::*;

use crate::component::common::ProgressBar;
use crate::style::{BLACK, CHINESE_WHITE, COMMON_RADIUS, ISABELLINE, WHITE};

#[derive(Declare)]
pub(super) struct AccountItem {
  name: CowArc<str>,
}

impl ComposeChild for AccountItem {
  type Child = Widget;

  fn compose_child(this: impl StateWriter<Value = Self>, child: Self::Child) -> impl WidgetBuilder {
    fn_widget! {
      @Column {
        @Text {
          text: $this.name.to_owned(),
          text_style: TypographyTheme::of(ctx!()).title_small.text.clone(),
        }
        @ { child }
      }
    }
  }
}

pub(super) fn w_email() -> impl WidgetBuilder {
  fn_widget! {
    @Row {
      justify_content: JustifyContent::SpaceBetween,
      // TODO: -8.? need check UI design.
      margin: EdgeInsets::only_top(-8.),
      @Row {
        item_gap: 10.,
        @Text {
          text: "Anonymous",
        }
        // TODO: ID show is optional.
        @TextSelectable {
          @Text {
            text: "ID: 123456",
            foreground: Palette::of(ctx!()).outline(),
          }
        }
      }
      @Button {
        cursor: CursorIcon::Hand,
        // TODO: check here UI design color.
        color: Color::RED,
        @ { Label::new("Logout") }
      }
    }
  }
}

pub(super) fn w_subscription() -> impl WidgetBuilder {
  fn_widget! {
    @ConstrainedBox {
      clamp: BoxClamp {
        min: Size::new(f32::INFINITY, 110.),
        max: Size::new(f32::INFINITY, 140.),
      },
      border_radius: COMMON_RADIUS,
      border: Border::all(BorderSide {
        width: 1.,
        color: Color::from_u32(CHINESE_WHITE).into(),
      }),
      padding: EdgeInsets::all(20.),
      margin: EdgeInsets::only_top(10.),
      // TODO: check here why need `Stack`? it's only one child.
      @Stack {
        @Row {
          @ { w_free_plan() }
          @Divider {
            direction: Direction::Vertical,
          }
          @ { w_subscription_plan() }
        }
      }
    }
  }
}

fn w_free_plan() -> impl WidgetBuilder {
  fn_widget! {
    @Row {
      item_gap: 16.,
      @Avatar {
        @ { Label::new("🚲") }
      }
      @ConstrainedBox {
        clamp: BoxClamp::fixed_width(270.),
        @Column {
          @ {
            w_plan_desc(
              "Free Plan".to_owned(),
              "Current Plan".to_owned(),
              Palette::of(ctx!()).on_surface_variant(),
              Color::from_u32(ISABELLINE),
            )
          }
          @ { w_quota_usage_amount() }
        }
      }
    }
  }
}

fn w_quota_usage_progress_bar(total: f32, completed: f32, tip: String) -> impl WidgetBuilder {
  fn_widget! {
    @Column {
      margin: EdgeInsets::new(0., 20., 0., 0.),
      item_gap: 6.,
      @Text {
        text: format!("{}/{} {}", completed as u32, total as u32, tip),
      }
      @ProgressBar {
        total,
        completed,
        bg_color: Color::from_u32(ISABELLINE),
        fg_color: Color::from_u32(BLACK),
        width: 220.,
        height: 9.,
        radius: 4.,
      }
    }
  }
}

fn w_quota_usage_amount() -> impl WidgetBuilder {
  fn_widget! {
    @Column {
      // text message usage
      @ { w_quota_usage_progress_bar(100., 20., "messages".to_owned()) }
      // image message usage
      @ { w_quota_usage_progress_bar(100., 10., "image".to_owned()) }
    }
  }
}

// TODO: here is user module, user quota is over.
fn w_quota_over() -> impl WidgetBuilder {
  fn_widget! {
    @Column {
      @Text {
        margin: EdgeInsets::new(10., 20., 0., 0.),
        overflow: Overflow::AutoWrap,
        text: "You have reached the maximum number of free requests. Please upgrade to a paid plan to continue using the service.",
      }
      @Link {
        url: "https://discord.gg/esyCEGhmq9",
        cursor: CursorIcon::Hand,
        @Text {
          text: "Polestar Discord",
          foreground: Color::from_u32(BLACK),
          background: Color::from_u32(WHITE),
          border_radius: COMMON_RADIUS,
          border: Border::all(BorderSide {
            width: 1.,
            color: Color::from_u32(CHINESE_WHITE).into(),
          }),
          padding: EdgeInsets::all(10.),
          margin: EdgeInsets::only_top(10.),
        }
      }
    }
  }
}

fn w_plan_desc(name: String, tag: String, fg: Color, bg: Color) -> impl WidgetBuilder {
  fn_widget! {
    @Row {
      item_gap: 8.,
      @Text {
        text: name,
        text_style: TypographyTheme::of(ctx!()).title_medium.text.clone(),
      }
      @Text {
        text: tag,
        foreground: fg,
        background: bg,
        border_radius: Radius::all(4.),
        padding: EdgeInsets::new(4., 8., 4., 8.),
        text_style: TypographyTheme::of(ctx!()).body_small.text.clone(),
      }
    }
  }
}

fn w_subscription_plan() -> impl WidgetBuilder {
  fn_widget! {
    @Row {
      margin: EdgeInsets::only_top(20.),
      item_gap: 16.,
      @Avatar {
        @ { Label::new("🚀") }
      }
      @Column {
        @ {
          w_plan_desc(
            "Subscription Plan".to_owned(),
            "Comming Soon".to_owned(),
            Color::from_u32(WHITE),
            Color::from_u32(BLACK),
          )
        }
        @Column {
          item_gap: 10.,
          margin: EdgeInsets::only_top(10.),
          @Text {
            padding: EdgeInsets::only_right(10.),
            overflow: Overflow::AutoWrap,
            text: "You can get more quota by subscribing to the plan.",
          }
        }
      }
    }
  }
}
