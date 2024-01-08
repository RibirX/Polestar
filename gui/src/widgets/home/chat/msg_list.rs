use polestar_core::model::{Channel, Msg, MsgCont, MsgRole};
use ribir::prelude::*;
use uuid::Uuid;

use crate::style::decorator::channel::message_style;
use crate::style::{GAINSBORO, WHITE};
use crate::theme::polestar_svg;
use crate::widgets::common::IconButton;
use crate::widgets::helper::send_msg;

use super::onboarding::w_msg_onboarding;

pub fn w_msg_list<S>(
  channel: S,
  quote_id: impl StateWriter<Value = Option<Uuid>>,
) -> impl WidgetBuilder
where
  S: StateWriter<Value = Channel>,
{
  fn_widget! {
    let scrollable_container = @VScrollBar {};

    let mut content_constrained_box = @ConstrainedBox {
      clamp: BoxClamp {
        min: Size::new(500., 0.),
        max: Size::new(800., f32::INFINITY),
      },
    };

    let scroll_to_bottom = move || {
      let _ = $scrollable_container.write();
      let scrollable_container = scrollable_container.clone_writer();
      watch!($content_constrained_box.layout_height())
        .distinct_until_changed()
        .take(1)
        .subscribe(move |layout_height| {
          let mut scrollable_container = $scrollable_container.write();
          scrollable_container.offset = -layout_height;
        });
    };

    scroll_to_bottom();

    watch!((
      $channel.msgs().len(),
      $channel.last_msg().map_or(0, |msg| msg.cont_size())
    ))
      .distinct_until_changed()
      .subscribe(move |_| {
        scroll_to_bottom();
      });

    @ConstrainedBox {
      clamp: BoxClamp::EXPAND_BOTH,
      @$scrollable_container {
        @$content_constrained_box {
          @Column {
            padding: EdgeInsets::all(16.),
            item_gap: 16.,
            @ { w_msg_onboarding() }
            @ {
              pipe! {
                let _ = || $channel.write();
                let channel_cloned = channel.clone_writer();
                let quote_id = quote_id.clone_writer();

                $channel.msgs().iter().map(move |m| {
                  let id = *m.id();
                  let msg = channel_cloned.split_writer(
                    move |channel| {
                      channel.msg(&id).expect("msg must be existed")
                    },
                    move |channel| {
                      channel.msg_mut(&id).expect("msg must be existed")
                    },
                  );
                  @ { w_msg(msg, quote_id.clone_writer()) }
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
          child.into_iter().enumerate().map(|(idx, ch)| {
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
              @ { ch }
            }
          })
        }
      }
    }
  }
}

fn w_msg<S, R, W>(msg: S, quote_id: impl StateWriter<Value = Option<Uuid>>) -> impl WidgetBuilder
where
  S: StateWriter<Value = Msg>,
  S::Writer: StateWriter<Value = Msg, OriginReader = R, OriginWriter = W>,
  S::OriginWriter: StateWriter<Value = Channel>,
  S::OriginReader: StateReader<Value = Channel>,
  R: StateReader<Value = Channel>,
  W: StateWriter<Value = Channel>,
  <S::Writer as StateWriter>::OriginWriter: StateWriter<Value = Channel>,
  <<S::Writer as StateWriter>::Writer as StateWriter>::OriginWriter: StateWriter<Value = Channel>,
  <<<S::Writer as StateWriter>::Writer as StateWriter>::Writer as StateWriter>::OriginWriter:
    StateWriter<Value = Channel>,
{
  fn_widget! {
    @ {
      pipe!($msg;).map(move |_| {
        let mut stack = @Stack {};

        let role = $msg.role().clone();
        let mut row = @Row {
          item_gap: 8.,
          reverse: matches!(role, MsgRole::User)
        };

        let msg_ops_anchor = {
          match role {
            MsgRole::User => @RelativeAnchor {
              anchor: pipe!(Anchor::right($row.layout_width() + 4.))
            },
            _ => @RelativeAnchor {
              anchor: pipe!(Anchor::left($row.layout_width() + 4.))
            },
          }
        };

        let retry_msg = msg.clone_writer();

        let role_2 = role.clone();
        let role_3 = role.clone();
        let msg_ops = @$msg_ops_anchor {
          visible: pipe! {
            $stack.mouse_hover() && !role_2.is_system()
          },
          @ {
            match role_3.clone() {
              MsgRole::User | MsgRole::Bot(_) => {
                let channel = msg.origin_writer();
                @MsgOps {
                  @ {
                    let quote_id = quote_id.clone_writer();
                    (!$channel.is_feedback()).then(move || {
                      @MsgOp {
                        cb: Box::new(move || {
                          *quote_id.write() = Some(*$msg.id())
                        }) as Box<dyn Fn()>,
                        @IconButton {
                          padding: EdgeInsets::all(4.),
                          size: IconSize::of(ctx!()).tiny,
                          @ { polestar_svg::REPLY }
                        }
                      }
                    })
                  }
                  @MsgOp {
                    cb: Box::new(move || {
                      if let Some(text) = $msg.cur_cont_ref().text() {
                        let clipboard = AppCtx::clipboard();
                        let _ = clipboard.borrow_mut().clear();
                        let _ = clipboard.borrow_mut().write_text(text);
                      }
                    }) as Box<dyn Fn()>,
                    @IconButton {
                      padding: EdgeInsets::all(4.),
                      size: IconSize::of(ctx!()).tiny,
                      @ { polestar_svg::CLIPBOARD }
                    }
                  }
                  @ {
                    let retry_msg = retry_msg.clone_writer();
                    let channel = msg.origin_writer().clone_writer();
                    ($msg.role().is_bot()).then(move || {
                      @MsgOp {
                        // TODO: cb code is messy, need to refactor.
                        cb: Box::new(move || {
                          let _ = || $retry_msg.write();
                          let _ = || $channel.write();
                          let source_id = {
                            let retry_msg_write = $retry_msg.write();
                            *retry_msg_write.meta().source_id().unwrap()
                          };
                          let msg_id = *$retry_msg.id();
                          let source_msg_text = $channel
                            .msg(&source_id)
                            .and_then(|msg| msg.cur_cont_ref().text().map(|text| text.to_owned()))
                            .unwrap_or_default().clone();
                          let (cur_idx, bot_id) = {
                            let mut retry_msg_write = $retry_msg.write();
                            let cont = MsgCont::init_text();
                            retry_msg_write.add_cont(cont);
                            let cur_idx = retry_msg_write.cur_idx();
                            let bot_id = retry_msg_write.role().bot().unwrap().clone();
                            (cur_idx, bot_id)
                          };
                          send_msg(channel.clone_writer(), source_msg_text, cur_idx, msg_id, bot_id);
                        }) as Box<dyn Fn()>,
                        @IconButton {
                          padding: EdgeInsets::all(4.),
                          size: IconSize::of(ctx!()).tiny,
                          @ { polestar_svg::RETRY }
                        }
                      }
                    })
                  }
                }.widget_build(ctx!())
              },
              _ => {
                @Void {}.widget_build(ctx!())
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
                @ { Label::new("A") }
              }
              @Column {
                @ {
                  let channel = msg.origin_writer().clone_writer();
                  $msg.role().bot().and_then(move |bot_id| {
                    $channel.app_info().map(|info| {
                      let bot = info.get_bot_or_default(Some(bot_id));
                      @Text { text: bot.name().to_owned() }
                    })
                  })
                }
                @ConstrainedBox {
                  clamp: BoxClamp {
                    min: Size::zero(),
                    max: Size::new(560., f32::INFINITY),
                  },
                  @ {
                    pipe!($msg.cur_idx()).map(move |_| {
                      let _msg_capture = || $msg.write();
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
                                w_msg_multi_rst(msg2.clone_writer())
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
                        $msg.role().clone()
                      )
                    })
                  }
                }
              }
            }
          }
          @ { msg_ops }
        }
      })
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

fn w_msg_multi_rst(msg: impl StateWriter<Value = Msg>) -> impl WidgetBuilder {
  fn_widget! {
    let scrollable_widget = @ScrollableWidget {
      scrollable: Scrollable::X,
    };
    let thumbnail_msg = msg.clone_writer();
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
              $msg.cont_list().iter().enumerate().map(|(idx, cont)| {
                let text = cont.text().map(|s| s.to_owned()).unwrap_or_default();
                let w_thumbnail = w_msg_thumbnail(text);
                @$w_thumbnail {
                  on_tap: move |_| {
                    let mut msg_write = $thumbnail_msg.write();
                    msg_write.switch_cont(idx);
                  }
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
        overflow: Overflow::AutoWrap,
        text_style: TypographyTheme::of(ctx!()).body_large.text.clone()
      }
    }
  }
}
