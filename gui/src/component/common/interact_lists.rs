use ribir::prelude::*;

use crate::style::{decorator::channel::highlight_style, GRAY_HIGHLIGHT};

#[derive(Declare)]
pub struct InteractList {
  #[declare(default = true)]
  need_highlight: bool,
  #[declare(default = 0 as usize)]
  active: usize,
  #[declare(skip)]
  highlight_rect_list: Vec<Rect>,
}

impl InteractList {
  fn derain(&mut self, count: usize) { self.highlight_rect_list.drain(count..); }
}

impl ComposeChild for InteractList {
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
          $this.need_highlight.then(||
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
              // TODO: check this code writing is why?
              let mut item_box = @ $item {};
              @PointerListener {
                on_tap: move |_| {
                  $this.write().active = idx;
                },
                on_performed_layout: move |_| {
                  $this.write().highlight_rect_list.push($item_box.layout_rect());
                },
                @ { item_box }
              }
            })
          }
        }
      }
    }
  }
}
