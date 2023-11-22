use ribir::prelude::*;

use super::app::AppGUI;
use crate::{
  oauth::{apple_login_uri, google_login_uri, microsoft_login_uri},
  style::WHITE,
  theme::polestar_svg,
  widgets::common::{Carousel, DoubleColumn, LeftColumn, RightColumn},
};

pub(super) fn w_login(app: impl StateWriter<Value = AppGUI>) -> impl WidgetBuilder {
  let logo_png_data = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/..",
    "/gui/static/icon128x128.png"
  ));
  fn_widget! {
    @DoubleColumn {
      fixed_width: 390.,
      @LeftColumn {
        @Column {
          margin: EdgeInsets::only_left(40.),
          h_align: HAlign::Center,
          v_align: VAlign::Center,
          item_gap: 10.,
          @SizedBox {
            margin: EdgeInsets::new(50., 0., 10., 0.),
            size: Size::splat(90.),
            @FittedBox {
              box_fit: BoxFit::Fill,
              @ {
                ShareResource::new(PixelImage::from_png(logo_png_data))
              }
            }
          }
          @Text {
            text: "Welcome to Polestar",
            text_style: TypographyTheme::of(ctx!()).headline_small.text.clone(),
          }
          @Text {
            text: "Log in to use our service.",
            text_style: TypographyTheme::of(ctx!()).title_medium.text.clone(),
          }
          @Column {
            margin: EdgeInsets::only_top(8.),
            item_gap: 8.,
            @LoginBtn {
              url: pipe! {
                if $app.data.need_login() {
                  microsoft_login_uri()
                } else {
                  String::new()
                }
              },
              label: "Log in with Microsoft",
              svg: polestar_svg::MICROSOFT_LOGIN,
            }
            @LoginBtn {
              url: pipe! {
                if $app.data.need_login() {
                  google_login_uri()
                } else {
                  String::new()
                }
              },
              label: "Log in with Google",
              svg: polestar_svg::GOOGLE_LOGIN,
            }
            @LoginBtn {
              url: pipe! {
                if $app.data.need_login() {
                  apple_login_uri()
                } else {
                  String::new()
                }
              },
              label: "Log in with Apple",
              svg: polestar_svg::APPLE_LOGIN,
            }
          }
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

#[derive(Declare)]
struct LoginBtn {
  url: CowArc<str>,
  label: CowArc<str>,
  svg: NamedSvg,
}

impl Compose for LoginBtn {
  fn compose(this: impl StateWriter<Value = Self>) -> impl WidgetBuilder {
    fn_widget! {
      @Link {
        url: $this.url.to_owned(),
        background: Color::from_u32(WHITE),
        @OutlinedButton {
          @ { $this.svg }
          @ { Label::new($this.label.to_owned()) }
        }
      }
    }
  }
}
