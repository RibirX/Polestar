use ribir::{material::InteractiveLayer, prelude::*};

use crate::style::{decorator::channel::highlight_style, WHITE};

#[derive(Declare)]
pub struct InteractiveList {
  #[declare(default = Color::from_u32(WHITE))]
  highlight_color: Color,
  #[declare(default = true)]
  highlight_visible: bool,
  #[declare(default = 0 as usize)]
  active: usize,
  #[declare(skip)]
  highlight_rect_list: Vec<Rect>,
}

impl InteractiveList {
  fn derain(&mut self, count: usize) { self.highlight_rect_list.drain(count..); }
}

impl ComposeChild for InteractiveList {
  type Child = Vec<Widget>;
  fn compose_child(this: impl StateWriter<Value = Self>, child: Self::Child) -> impl WidgetBuilder {
    let child_count = child.len();
    fn_widget! {
      let block = @SizedBox {
        size: pipe!($this.active).map(move |idx| {
          if let Some(rect) = &$this.highlight_rect_list.get(idx) {
            rect.size
          } else {
            Size::zero()
          }
        }).value_chain(|s| s.distinct_until_changed()),
      };

      let highlight = highlight_style(block.widget_build(ctx!()));

      @Stack {
        @ {
          $this.highlight_visible.then(||
            @$highlight {
              top_anchor: pipe!($this.active).map(move |idx| {
                if let Some(rect) = &$this.highlight_rect_list.get(idx) {
                  rect.origin.y
                } else {
                  0.
                }
              }).value_chain(|s| s.distinct_until_changed())
            }
          )
        }
        @Lists {
          on_performed_layout: move |_| {
            $this.write().derain(child_count);
          },
          @ {
            child.into_iter().enumerate().map(move |(idx, item)| {
              let mut item_box = @ $item {
                cursor: CursorIcon::Hand,
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
                  on_tap: move |_| {
                    $this.write().active = idx;
                  },
                  @ { item_box }
                }
              }
            })
          }
        }
      }
    }
  }
}
