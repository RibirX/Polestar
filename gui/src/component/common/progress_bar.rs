use ribir::prelude::*;

#[derive(Declare)]
pub struct ProgressBar {
  pub total: f32,
  pub completed: f32,
  #[declare(default = Palette::of(ctx!()).surface_variant())]
  pub bg_color: Color,
  #[declare(default = Palette::of(ctx!()).on_surface_variant())]
  pub fg_color: Color,
  pub width: f32,
  #[declare(default = 8.)]
  pub height: f32,
  #[declare(default = 0.)]
  pub radius: f32,
}

impl Compose for ProgressBar {
  fn compose(this: impl StateWriter<Value = Self>) -> impl WidgetBuilder {
    fn_widget! {
      @Clip {
        clip: ClipType::Path(Path::rect_round(
          &Rect::new(Point::zero(), Size::new($this.width, $this.height)),
          &Radius::all($this.radius)),
        ),
        @Container {
          border_radius: Radius::all($this.radius),
          background: $this.bg_color,
          size: Size::new($this.width, $this.height),
          @SizedBox {
            border_radius: Radius::all($this.radius),
            background: $this.fg_color,
            size: pipe!(Size::new(
              $this.width * ($this.completed / $this.total).min(1.),
              $this.height,
            )),
          }
        }
      }
    }
  }
}
