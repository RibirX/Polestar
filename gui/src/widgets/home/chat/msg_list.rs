use polestar_core::model::{ChannelId, MsgCont, MsgId, MsgRole};
use ribir::prelude::*;
use uuid::Uuid;

use crate::style::decorator::channel::message_style;
use crate::style::{GAINSBORO, WHITE};
use crate::theme::polestar_svg;
use crate::widgets::app::Chat;
use crate::widgets::common::IconButton;
use crate::widgets::helper::send_msg;

use super::onboarding::w_msg_onboarding;

pub fn w_msg_list(
  chat: impl StateWriter<Value = dyn Chat>,
  channel_id: ChannelId,
  quote_id: impl StateWriter<Value = Option<Uuid>>,
) -> impl WidgetBuilder {
  fn_widget! {
    let channel = chat.map_reader(move |chat| chat.channel(&channel_id).unwrap());
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
                let _ = || $chat.write();
                let quote_id = quote_id.clone_writer();
                let channel_id = *$channel.id();
                let chat = chat.clone_writer();
                $channel.msgs().iter().map(move |m| {
                  let id = *m.id();
                  @ { w_msg(chat.clone_writer(), channel_id, id, quote_id.clone_writer()) }
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

fn w_msg(
  chat: impl StateWriter<Value = dyn Chat>,
  channel_id: ChannelId,
  msg_id: MsgId,
  quote_id: impl StateWriter<Value = Option<Uuid>>,
) -> impl WidgetBuilder {
  fn_widget! {
    @ {
        let mut stack = @Stack {};
        let chat_ref = $chat;
        let msg = chat_ref.msg(&channel_id, &msg_id).unwrap();
        let role = msg.role().clone();
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

        let role_2 = role.clone();
        let role_3 = role.clone();
        let msg_ops = @$msg_ops_anchor {
          visible: pipe! {
            $stack.mouse_hover() && !role_2.is_system()
          },
          @ {
            match role_3.clone() {
              MsgRole::User | MsgRole::Bot(_) => {
                @MsgOps {
                  @ {
                    let quote_id = quote_id.clone_writer();
                    let quote_fn = Box::new(move || {
                      *quote_id.write() = Some(msg_id)
                    }) as Box<dyn Fn()>;
                    let is_feedback = $chat.channel(&channel_id).map_or(false, |ch| ch.is_feedback());
                    (!is_feedback).then(move || {
                      @MsgOp {
                        cb: quote_fn,
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
                      let chat = $chat;
                      let msg = chat.msg(&channel_id, &msg_id).unwrap();
                      if let Some(text) = msg.cur_cont_ref().text() {
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
                    let source_id = msg.meta().source_id().cloned();
                    let bot_id = msg.role().bot().cloned();
                    let msg_id = *msg.id();
                    let chat = chat.clone_writer();
                    let role = msg.role().clone();
                    (role.is_bot()).then(move || {
                      let _hint = || $chat.write();
                      @MsgOp {
                        // TODO: cb code is messy, need to refactor.
                        cb: Box::new(move || {
                          let _hint = || $chat.write();
                          let (source_msg, idx) = {
                            let mut chat = $chat.write();
                            (
                              chat
                                .msg(&channel_id, &source_id.unwrap())
                                .and_then(|msg| msg.cur_cont_ref().text().map(|text| text.to_owned()))
                                .unwrap_or_default().clone(),
                              chat.add_msg_cont(&channel_id, &msg_id, MsgCont::init_text()).unwrap()
                            )
                          };
                          $chat.write().switch_cont(&channel_id, &msg_id, idx);
                          send_msg(chat.clone_writer(), channel_id,  msg_id, idx, bot_id.clone().unwrap(), source_msg);
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
            h_align: match msg.role() {
              MsgRole::User => HAlign::Right,
              _ => HAlign::Left,
            },
            @$row {
              @Avatar {
                @ { Label::new("A") }
              }
              @Column {
                @ {
                  msg.role().bot().map(move |bot_id| {
                    let chat = $chat;
                    let bot = chat.info().get_bot_or_default(Some(bot_id));
                    @Text { text: bot.name().to_owned() }
                  })
                }
                @ConstrainedBox {
                  clamp: BoxClamp {
                    min: Size::zero(),
                    max: Size::new(560., f32::INFINITY),
                  },
                  @ {
                      let default_txt = String::new();
                      // TODO: support Image Type.
                      let text = msg
                        .cur_cont_ref()
                        .text()
                        .unwrap_or_else(|| &default_txt).to_owned();
                      let quote_id = msg.meta().quote_id().cloned();
                      message_style(
                        @Column {
                          @ { w_msg_quote(&*$chat, &channel_id, quote_id) }
                          @ {
                            let chat = chat.clone_writer();
                            (msg.cont_list().len() > 1).then(move || {
                              w_msg_multi_rst(chat, channel_id, msg_id)
                            })
                          }
                          @TextSelectable {
                            @Text {
                              text,
                              overflow: Overflow::AutoWrap,
                              text_style: TypographyTheme::of(ctx!()).body_large.text.clone()
                            }
                          }
                        }.widget_build(ctx!()),
                        msg.role().clone()
                      )
                  }
                }
              }
            }
          }
          @ { msg_ops }
        }
    }
  }
}

fn w_msg_quote(chat: &dyn Chat, channel_id: &ChannelId, quote_id: Option<MsgId>) -> Option<impl WidgetBuilder>
{
  let quote_text = quote_id.and_then(move |id| {
    chat.msg(channel_id, &id).and_then(|msg| msg.cur_cont_ref().text().map(|s| s.to_owned()))
  });

  quote_text.map(|text| {
    fn_widget! {
      @Text {
        text,
      }
    }
  })
}

fn w_msg_multi_rst(
  chat: impl StateWriter<Value = dyn Chat>,
  channel_id: ChannelId,
  msg_id: MsgId) -> impl WidgetBuilder {
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
              $chat.msg(&channel_id, &msg_id).unwrap().cont_list().iter().enumerate().map(|(idx, cont)| {
                let text = cont.text().map(|s| s.to_owned()).unwrap_or_default();
                let w_thumbnail = w_msg_thumbnail(text);
                @$w_thumbnail {
                  on_tap: move |_| {
                    $chat.write().switch_cont(&channel_id, &msg_id, idx);
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
