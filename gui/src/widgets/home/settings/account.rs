
use ribir::prelude::*;

use crate::req::query_quota;
use crate::style::{BLACK, CHINESE_WHITE, COMMON_RADIUS, ISABELLINE, WHITE};
use crate::widgets::app::AppGUI;
use crate::widgets::common::ProgressBar;

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

pub(super) fn w_email(app: &impl StateWriter<Value = AppGUI>) -> impl WidgetBuilder {
  let app = app.clone_writer();
  fn_widget! {
    @Row {
      justify_content: JustifyContent::SpaceBetween,
      // TODO: -8.? need check UI design.
      margin: EdgeInsets::only_top(-8.),
      @Row {
        item_gap: 10.,
        @Text {
          text: $app.data.info().user().and_then(|u| u.email()).cloned().unwrap_or("Anonymous".to_string()),
        }
        @TextSelectable {
          @Text {
            text: $app.data.info().user().map(|user| format!("ID: {}", user.uid())).unwrap_or_default(),
            foreground: Palette::of(ctx!()).outline(),
          }
        }
      }
      @Button {
        cursor: CursorIcon::Pointer,
        color: Color::RED,
        on_tap: move |_| {
          $app.write().data.logout();
          $app.write().navigate_to("/login");
        },
        @ { Label::new("Logout") }
      }
    }
  }
}

pub(super) fn w_subscription(app: impl StateWriter<Value = AppGUI>) -> impl WidgetBuilder {
  fn_widget! {
    println!("subscription");
    let _ = || $app.write();
    let spawn_app = app.clone_writer();
    let token = $spawn_app.data.info().user().and_then(|user| user.token().map(|s| s.to_owned()));
    let _ = AppCtx::spawn_local(async move {
      let quota = query_quota(token).await.ok();
      spawn_app.silent().data.info_mut().user_mut().unwrap().set_quota(quota);
    });
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
          @ { w_free_plan(app.clone_reader()) }
          @Divider {
            direction: Direction::Vertical,
          }
          @ { w_subscription_plan() }
        }
      }
    }
  }
}

fn w_free_plan(app: impl StateReader<Value = AppGUI>) -> impl WidgetBuilder {
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
          @ {
            w_quota_usage_amount(app.clone_reader())
          }
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

fn w_quota_usage_amount(app: impl StateReader<Value = AppGUI>) -> impl WidgetBuilder {
  fn_widget! {
    @ {
      if $app.data.info().user().map(|user| user.quota().map(|quota| quota.is_over()).unwrap_or_default()).unwrap_or_default() {
        w_quota_over().widget_build(ctx!())
      } else {
        w_quota_usage(app.clone_reader()).widget_build(ctx!())
      }
    }
  }
}

fn w_quota_usage(app: impl StateReader<Value = AppGUI>) -> impl WidgetBuilder {
  fn_widget! {
    @Column {
      // text message usage
      @ {
        let text_total = $app.data.info().user().map(|user| user.quota().map(|quota| quota.max_texts()).unwrap_or_default()).unwrap_or_default();
        let text_used = $app.data.info().user().map(|user| user.quota().map(|quota| quota.text_used()).unwrap_or_default()).unwrap_or_default();
        w_quota_usage_progress_bar(text_total, text_used, "messages".to_owned())
      }
      // image message usage
      // @ { w_quota_usage_progress_bar(100., 10., "image".to_owned()) }
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
        cursor: CursorIcon::Pointer,
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
      margin: EdgeInsets::only_left(20.),
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
