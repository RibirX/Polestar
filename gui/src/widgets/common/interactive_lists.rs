use ribir::{material::InteractiveLayer, prelude::*};

use crate::style::{decorator::channel::highlight_style, WHITE};

#[derive(Declare)]
pub struct InteractiveList {
  #[declare(default = Color::from_u32(WHITE))]
  highlight_color: Color,
  #[declare(default = true)]
  highlight_visible: bool,
  #[declare(default = 0_usize)]
  active: usize,
  #[declare(skip)]
  highlight_rect_list: Vec<Rect>,
}

impl ComposeChild for InteractiveList {
  type Child = BoxPipe<Vec<Widget>>;
  fn compose_child(this: impl StateWriter<Value = Self>, child: Self::Child) -> impl WidgetBuilder {
    fn_widget! {
      let mut vscrollbar = @VScrollBar {};

      watch!($this.active)
        .distinct_until_changed()
        .subscribe(move |active| {
          if let Some(rect) = &$this.highlight_rect_list.get(active) {
            let offset = $vscrollbar.offset;
            let y = rect.origin.y + offset;
            if y < 0. || y > $vscrollbar.layout_height() {
              $vscrollbar.write().offset = -rect.origin.y;
            }
          }
        });

      @$vscrollbar {
        @Stack {
          @ {
            pipe! {
              let block = @SizedBox {
                size: pipe!($this.active).map(move |idx| {
                  if let Some(rect) = &$this.highlight_rect_list.get(idx) {
                    rect.size
                  } else {
                    Size::zero()
                  }
                }).value_chain(|s| s.distinct_until_changed().box_it()),
              };
              let highlight = highlight_style(block.widget_build(ctx!()));
              $this.highlight_visible.then(||
                @$highlight {
                  anchor: pipe!($this.active).map(move |idx| {
                    if let Some(rect) = &$this.highlight_rect_list.get(idx) {
                      Anchor::top(rect.origin.y)
                    } else {
                      Anchor::top(0.)
                    }
                  }).value_chain(|s| s.distinct_until_changed().box_it())
                }
              )
            }
          }
          @Lists {
            @ {
              child.into_pipe().map(move |child| {
                $this.silent().highlight_rect_list.clear();
                child.into_iter().enumerate().map(move |(idx, item)| {
                  let mut item_box = @ $item {
                    cursor: CursorIcon::Pointer,
                  };
                  @BoxDecoration {
                    // TODO: here display has 4px offset in real case.
                    margin: EdgeInsets::vertical(4.),
                    on_performed_layout: move |ctx| {
                      let rect = $item_box.layout_rect();
                      let offset = ctx.map_to_parent(rect.origin);
                      $this.write().highlight_rect_list.push(Rect::new(offset, rect.size));
                    },
                    // TODO: here depends on material theme, need split it.
                    @InteractiveLayer {
                      color: Palette::of(ctx!()).base_of(&$this.highlight_color),
                      border_radii: Radius::all(8.),
                      @$item_box {
                        on_tap: move |_| {
                          $this.write().active = idx;
                        },
                      }
                    }
                  }
                })
              })
            }
          }
        }
      }
    }
  }
}
