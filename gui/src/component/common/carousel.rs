use ribir::prelude::*;

use crate::{
  component::common::carousel,
  style::{BLACK, LIGHT_SILVER_FF},
};

pub struct GraphicIntro {
  image: ShareResource<PixelImage>,
  title: Option<CowArc<str>>,
  desc: Option<CowArc<str>>,
}

impl GraphicIntro {
  pub fn new(
    image: ShareResource<PixelImage>,
    title: Option<impl Into<CowArc<str>>>,
    desc: Option<impl Into<CowArc<str>>>,
  ) -> Self {
    Self {
      image,
      title: title.map(Into::into),
      desc: desc.map(Into::into),
    }
  }
}

#[derive(Declare)]
pub struct Carousel {
  contents: Vec<GraphicIntro>,
  #[declare(default = 0 as usize)]
  cur_idx: usize,
  #[declare(default = 5 as u64)]
  interval: u64,
  #[declare(default = Size::new(560., 380.))]
  banner_size: Size,
  #[declare(default = BoxFit::Cover)]
  banner_fit: BoxFit,
}

impl Compose for Carousel {
  fn compose(this: impl StateWriter<Value = Self>) -> impl WidgetBuilder {
    fn_widget! {
      let mut fitted_banner = @FittedBox {
        box_fit: $this.banner_fit,
      };
      let dots = w_dots(this.clone_writer());
      @Stack {
        fit: StackFit::Passthrough,
        @Column {
          v_align: VAlign::Center,
          @Column {
            @ { w_title(this.clone_reader()) }
            @ { w_intro(this.clone_reader()) }
            @Container {
              size: $this.banner_size,
              @Clip {
                v_align: VAlign::Center,
                h_align: HAlign::Center,
                clip: ClipType::Path(
                  Path::rect_round(
                    &Rect::new(Point::zero(), $fitted_banner.layout_size()),
                    &Radius::all(10.),
                  ),
                ),
                @$fitted_banner {
                  @ { $this.contents.get($this.cur_idx).map(|intro| intro.image.clone()).unwrap() }
                }
              }
            }
          }
          @$dots {
            margin: EdgeInsets::only_top(15.)
          }
        }
      }
    }
  }
}

fn w_cur_banner(carousel: impl StateReader<Value = Carousel>) -> Option<impl WidgetBuilder> {
  let carousel_ref = carousel.read();
  let cur_intro = carousel_ref.contents.get(carousel_ref.cur_idx);
  cur_intro.map(|intro| intro.image.clone()).map(|image| {
    fn_widget! {
      @ { image }
    }
  })
}

fn w_title(carousel: impl StateReader<Value = Carousel>) -> Option<impl WidgetBuilder> {
  let carousel_ref = carousel.read();
  carousel_ref.contents[carousel_ref.cur_idx]
    .title
    .clone()
    .map(|text| {
      fn_widget! {
        @Text {
          text,
          text_style: TypographyTheme::of(ctx!()).title_large.text.clone(),
          margin: EdgeInsets::only_bottom(10.),
        }
      }
    })
}

fn w_intro(carousel: impl StateReader<Value = Carousel>) -> Option<impl WidgetBuilder> {
  let carousel_ref = carousel.read();
  carousel_ref.contents[carousel_ref.cur_idx]
    .desc
    .clone()
    .map(|text| {
      fn_widget! {
        @Text {
          text,
          text_style: TypographyTheme::of(ctx!()).title_medium.text.clone(),
          overflow: Overflow::AutoWrap,
          margin: EdgeInsets::only_bottom(10.),
        }
      }
    })
}

fn w_dots(carousel: impl StateWriter<Value = Carousel>) -> impl WidgetBuilder {
  fn_widget! {
    @Row {
      h_align: HAlign::Center,
      item_gap: 10.,
      @ {
        pipe! {
          (0..$carousel.contents.len()).map(move |idx| {
            @Dot {
              on_tap: move |_| {
                if $carousel.cur_idx != idx {
                  $carousel.write().cur_idx = idx;
                }
              },
              is_active: idx == $carousel.cur_idx,
            }
          })
        }
      }
    }
  }
}

#[derive(Declare)]
struct Dot {
  is_active: bool,
}

impl Compose for Dot {
  fn compose(this: impl StateWriter<Value = Self>) -> impl WidgetBuilder {
    fn_widget! {
      @Container {
        size: Size::splat(12.),
        cursor: CursorIcon::Hand,
        border_radius: Radius::all(6.),
        background: Color::from_u32(LIGHT_SILVER_FF),
        @ {
          pipe! {
            ($this.is_active).then(|| {
              @Container {
                v_align: VAlign::Center,
                h_align: HAlign::Center,
                size: Size::splat(6.),
                border_radius: Radius::all(3.),
                background: Color::from_u32(BLACK)
              }
            })
          }
        }
      }
    }
  }
}
