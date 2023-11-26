use polestar_core::model::{Channel, Msg, MsgRole};
use ribir::prelude::*;

use crate::style::decorator::channel::message_style;
use crate::style::{GAINSBORO, WHITE};
use crate::widgets::common::IconButton;

use super::onboarding::w_msg_onboarding;

pub fn w_msg_list<S>(channel: S) -> impl WidgetBuilder
where
  S: StateWriter<Value = Channel>,
{
  fn_widget! {
    let scrollable_container = @VScrollBar {};

    @ConstrainedBox {
      clamp: BoxClamp::EXPAND_BOTH,
      @$scrollable_container {
        @ConstrainedBox {
          clamp: BoxClamp {
            min: Size::new(500., 0.),
            max: Size::new(800., f32::INFINITY),
          },
          @Column {
            padding: EdgeInsets::all(16.),
            item_gap: 16.,
            @ { w_msg_onboarding() }
            @ {
              pipe! {
                let _ = || $channel.write();
                let channel2 = channel.clone_writer();
                $channel.msgs().iter().enumerate().map(move |(idx, _)| {
                  let msg = channel2.split_writer(
                    move |channel| {
                      channel.msgs().get(idx).expect("msg must be existed")
                    },
                    move |channel| {
                      channel.msgs_mut().get_mut(idx).expect("msg must be existed")
                    },
                  );
                  @ { w_msg(msg) }
                }).collect::<Vec<_>>()
              }
            }
          }
        }
      }
    }
  }
}

#[derive(Declare)]
struct MsgOps;

#[derive(Declare)]
struct MsgOp {
  cb: Box<dyn Fn()>,
}

impl ComposeChild for MsgOp {
  type Child = Widget;
  fn compose_child(this: impl StateWriter<Value = Self>, child: Self::Child) -> impl WidgetBuilder {
    fn_widget! {
      @$child {
        on_tap: move |_| {
          let cb = &$this.write().cb;
          cb();
        }
      }
    }
  }
}

impl ComposeChild for MsgOps {
  type Child = Vec<Pair<State<MsgOp>, Widget>>;
  fn compose_child(_: impl StateWriter<Value = Self>, child: Self::Child) -> impl WidgetBuilder {
    fn_widget! {
      @Row {
        border_radius: Radius::all(4.),
        border: Border::all(BorderSide {
          color: Color::from_u32(GAINSBORO).into(),
          width: 1.,
        }),
        @ {
          child.into_iter().enumerate().map(|(idx, c)| {
            let left_border = (idx != 0).then(|| {
              BoxDecoration {
                border: Some(Border::only_left(BorderSide {
                  color: Color::from_u32(GAINSBORO).into(),
                  width: 1.,
                })),
                ..<_>::default()
              }
            });
            @$left_border {
              @ { c }
            }
          })
        }
      }
    }
  }
}

fn w_msg<S, R, W>(msg: S) -> impl WidgetBuilder
where
  S: StateWriter<Value = Msg>,
  S::Writer: StateWriter<Value = Msg, OriginReader = R, OriginWriter = W>,
  S::OriginWriter: StateWriter<Value = Channel>,
  S::OriginReader: StateReader<Value = Channel>,
  R: StateReader<Value = Channel>,
  W: StateWriter<Value = Channel>,
{
  let channel = msg.origin_writer().clone_writer();
  fn_widget! {
    let mut stack = @Stack {};

    let mut row = @Row {
      item_gap: 8.,
      reverse: match $msg.role() {
        MsgRole::User => true,
        _ => false,
      },
    };

    let msg_ops_anchor = {
      match $msg.role() {
        MsgRole::User => @RightAnchor {
          right_anchor: pipe!($row.layout_width() + 4.)
        },
        _ => @LeftAnchor {
          left_anchor: pipe!($row.layout_width() + 4.)
        },
      }
    };

    let msg_ops = @$msg_ops_anchor {
      @MsgOps {
        visible: pipe! {
          $stack.mouse_hover() && $msg.role().is_system()
        },
        @MsgOp {
          cb: Box::new(|| {
            println!("add");
          }) as Box<dyn Fn()>,
          @IconButton {
            @ { svgs::ADD }
          }
        }
        @MsgOp {
          cb: Box::new(|| {
            println!("close");
          }) as Box<dyn Fn()>,
          @IconButton {
            @ { svgs::CLOSE }
          }
        }
      }
    };

    @$stack {
      @Row {
        h_align: match $msg.role() {
          MsgRole::User => HAlign::Right,
          _ => HAlign::Left,
        },
        @$row {
          @Avatar {
            // TODO: use bot avatar.
            @ { Label::new("A") }
          }
          @ConstrainedBox {
            clamp: BoxClamp {
              min: Size::zero(),
              max: Size::new(560., f32::INFINITY),
            },
            @ {
              let default_txt = String::new();
              // TODO: support Image Type.
              let text = $msg
                .cur_cont_ref()
                .text()
                .unwrap_or_else(|| &default_txt).to_owned();
              let msg2 = msg.clone_writer();
              message_style(
                @Column {
                  @ { w_msg_quote(msg2) }
                  @ {
                    let msg2 = msg.clone_writer();
                    pipe! {
                      ($msg.cont_list().len() > 1).then(|| {
                        w_msg_multi_rst(msg2.clone_reader())
                      })
                    }
                  }
                  @TextSelectable {
                    @Text {
                      text,
                      overflow: Overflow::AutoWrap,
                      text_style: TypographyTheme::of(ctx!()).body_large.text.clone()
                    }
                  }
                }.widget_build(ctx!()),
                *$msg.role()
              )
            }
          }
        }
      }
      @ { msg_ops }
    }
  }
}

fn w_msg_quote<S>(msg: S) -> Option<impl WidgetBuilder>
where
  S: StateWriter<Value = Msg>,
  S::OriginWriter: StateWriter<Value = Channel>,
{
  let channel = msg.origin_writer().clone_writer();
  let msg_state = msg.read();
  let quote_id = msg_state.meta().quote_id();

  let quote_text = quote_id.and_then(move |id| {
    let channel_state = channel.read();
    let msg = channel_state.msgs().iter().find(|msg| msg.id() == id);
    msg.and_then(|msg| msg.cur_cont_ref().text().map(|s| s.to_owned()))
  });

  quote_text.map(|text| {
    fn_widget! {
      @Text {
        text,
      }
    }
  })
}

fn w_msg_multi_rst(msg: impl StateReader<Value = Msg>) -> impl WidgetBuilder {
  fn_widget! {
    let scrollable_widget = @ScrollableWidget {
      scrollable: Scrollable::X,
    };
    @Row {
      @Visibility {
        @Void {}
      }
      @Expanded {
        flex: 1.,
        @$scrollable_widget {
          @Row {
            item_gap: 8.,
            @ {
              $msg.cont_list().iter().map(|cont| {
                let text = cont.text().map(|s| s.to_owned()).unwrap_or_else(|| String::new());
                @ {
                  w_msg_thumbnail(text)
                }
              }).collect::<Vec<_>>()
            }
          }
        }
      }
      @Visibility {
        @Void {}
      }
    }
  }
}

fn w_msg_thumbnail(text: String) -> impl WidgetBuilder {
  fn_widget! {
    @SizedBox {
      size: Size::new(150., 60.),
      background: Color::from_u32(WHITE),
      @Text {
        text,
        text_style: TypographyTheme::of(ctx!()).body_large.text.clone()
      }
    }
  }
}
