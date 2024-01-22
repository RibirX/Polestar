use ribir::prelude::*;

use crate::style::{ALICE_BLUE, LIGHT_SILVER_15};

#[derive(Declare)]
struct MsgOnboardingContainer {
  name: String,
  avatar: ShareResource<PixelImage>,
}

impl ComposeChild for MsgOnboardingContainer {
  type Child = Vec<Widget>;
  fn compose_child(this: impl StateWriter<Value = Self>, child: Self::Child) -> impl WidgetBuilder {
    fn_widget! {
      let path = Path::rect_round(
        &Rect::from_size(IconSize::of(ctx!()).medium),
        &Radius::all(20.),
      );

      let avatar = @Clip {
        clip: ClipType::Path(path),
        @Container {
          size: IconSize::of(ctx!()).medium,
          @FittedBox {
            box_fit: BoxFit::Contain,
            @ { $this.avatar.clone() }
          }
        }
      };

      let msg = @ConstrainedBox {
        clamp: BoxClamp {
          min: Size::zero(),
          max: Size::new(560., f32::INFINITY),
        },
        @Column {
          @Text { text: $this.name.clone() }
          @Column {
            item_gap: 10.,
            padding: EdgeInsets::new(12., 14., 12., 14.),
            border_radius: Radius::all(8.),
            background: Color::from_u32(ALICE_BLUE),
            border: Border::all(BorderSide {
              color: Color::from_u32(LIGHT_SILVER_15).into(),
              width: 1.,
            }),
            @ { child }
          }
        }
      };

      @Row {
        align_items: Align::Start,
        h_align: HAlign::Stretch,
        anchor: Anchor::left(0.),
        item_gap: 8.,
        @ { avatar }
        @ { msg }
      }
    }
  }
}

#[derive(Declare)]
struct MsgOnboarding {
  text: String,
}

impl ComposeChild for MsgOnboarding {
  type Child = Widget;
  fn compose_child(this: impl StateWriter<Value = Self>, child: Self::Child) -> impl WidgetBuilder {
    fn_widget! {
      @Column {
        @Text {
          margin: EdgeInsets::only_bottom(4.),
          overflow: Overflow::AutoWrap,
          text: $this.text.clone()
        }
        @ { child }
      }
    }
  }
}

#[derive(Declare)]
struct MsgOnboardingItem {
  emoji: CowArc<str>,
  text: String,
}

impl Compose for MsgOnboardingItem {
  fn compose(this: impl StateWriter<Value = Self>) -> impl WidgetBuilder {
    fn_widget! {
      @Row {
        @Avatar {
          @ { Label::new($this.emoji.to_owned()) }
        }
        @Text {
          text: $this.text.clone(),
          overflow: Overflow::AutoWrap,
        }
      }
    }
  }
}

pub fn w_msg_onboarding() -> impl WidgetBuilder {
  fn_widget! {
    @MsgOnboardingContainer {
      name: "Polestar".to_owned(),
      avatar: ShareResource::new(
        PixelImage::from_png(
          include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/..",
            "/gui/static/outlined_logo.png"
          ))
        )
      ),
      @MsgOnboarding {
        text: "Welcome to Polestar!".to_owned(),
        @Link {
          cursor: CursorIcon::Pointer,
          url: "https://www.polestarchat.com/",
          @Text {
            text: "polestarchat.com".to_owned()
          }
        }
      }
    }
  }
}
