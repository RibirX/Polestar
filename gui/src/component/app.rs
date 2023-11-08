use polestar_core::{
  load_bot_cfg_file,
  model::{AppData, Channel},
};
use ribir::prelude::*;
use ribir_algo::Sc;
use uuid::Uuid;

use super::{
  common::{Modal, PartialPath, Route, Router, Tooltip},
  home::w_home,
  login::w_login,
  permission::w_permission,
};
use crate::{
  style::{CHINESE_WHITE, COMMON_RADIUS, CULTURED_F7F7F5_FF},
  theme::polestar_theme,
};

pub struct AppGUI {
  cur_router_path: String,
  pub data: AppData,
  modify_channel_id: Option<Uuid>,
  tooltip: Option<String>,
}

impl AppGUI {
  pub fn new(data: AppData) -> Self {
    Self {
      data,
      // TODO: launch need confirm current router by user login status.
      // It can get by open app url, like this: PoleStarChat://ribir.org/home/chat
      cur_router_path: "/home/chat".to_owned(),
      modify_channel_id: None,
      tooltip: None,
    }
  }

  pub fn cur_router_path(&self) -> &str { &self.cur_router_path }

  pub fn navigate_to(&mut self, path: &str) { self.cur_router_path = path.to_owned(); }

  pub fn tooltip(&self) -> &Option<String> { &self.tooltip }

  pub fn set_tooltip(&mut self, tooltip: Option<&str>) {
    self.tooltip = tooltip.map(|s| s.to_owned());
  }

  pub fn modify_channel_id(&self) -> &Option<Uuid> { &self.modify_channel_id }

  pub fn set_modify_channel_id(&mut self, modify_channel_id: Option<Uuid>) {
    self.modify_channel_id = modify_channel_id;
  }
}

impl Compose for AppGUI {
  fn compose(this: impl StateWriter<Value = Self>) -> impl WidgetBuilder {
    fn_widget! {
      @ThemeWidget {
        // Polestar custom theme.
        theme: Sc::new(Theme::Inherit(polestar_theme())),
        @ {
          Box::new(fn_widget! {
            @Stack {
              on_mounted: move |_| {
                $this.write().set_tooltip(Some("hello world"));
              },
              @Router {
                cur_path: pipe!($this.cur_router_path().to_owned()),
                @Route {
                  path: PartialPath::new("/login", 0),
                  @ { w_login(this.clone_writer()) }
                }
                @Route {
                  path: PartialPath::new("/permission", 0),
                  @ { w_permission(this.clone_writer()) }
                }
                @Route {
                  path: PartialPath::new("/home", 0),
                  @ { w_home(this.clone_writer()) }
                }
              }
              @ {
                pipe! {
                  w_tooltip($this.tooltip())
                }
              }
              @ {
                let this2 = this.clone_writer();
                pipe! {
                  ($this.modify_channel_id()).map(|modify_channel_id| {
                    w_modify_channel_modal(this2.clone_writer(), modify_channel_id)
                  })
                }
              }
            }
          })
        }
      }
    }
  }
}

fn w_tooltip(tooltip: &Option<String>) -> Option<impl WidgetBuilder> {
  tooltip.to_owned().map(|tooltip| {
    fn_widget! {
      @Tooltip {
        content: tooltip,
      }
    }
  })
}

fn w_mode_options<S>(channel: S) -> impl WidgetBuilder
where
  S: StateReader<Value = Channel>,
{
  fn_widget! {
    @ConstrainedBox {
      clamp: BoxClamp {
        min: Size::new(500., 40.),
        max: Size::new(500., 65.),
      },
      cursor: CursorIcon::Hand,
      padding: EdgeInsets::all(10.),
      border: Border::all(BorderSide {
        width: 1.,
        color: Color::from_u32(CHINESE_WHITE).into(),
      }),
      border_radius: COMMON_RADIUS,
      @Row {
        align_items: Align::Center,
        @Checkbox {
          checked: false,
        }
        @Column {
          margin: EdgeInsets::only_left(6.),
          @Text {
            margin: EdgeInsets::only_bottom(4.),
            text: "Balanced",
            text_style: TypographyTheme::of(ctx!()).title_small.text.clone(),
          }
          @Text {
            text: "This mode automatically uses the advanced model and you will get more accurate answers but costs will increase.",
            overflow: Overflow::AutoWrap,
          }
        }
      }
    }
  }
}

fn w_modify_channel_modal(
  app: impl StateWriter<Value = AppGUI>,
  channel_id: Uuid,
) -> impl WidgetBuilder {
  let channel = app.split_writer(
    move |app| {
      app
        .data
        .get_channel(&channel_id)
        .expect("channel not found")
    },
    move |app| {
      app
        .data
        .get_channel_mut(&channel_id)
        .expect("channel not found")
    },
  );

  fn_widget! {
    let channel_rename = @Input {};
    let balanced_mode = w_mode_options(channel.clone_reader());
    let performance_mode = w_mode_options(channel.clone_reader());

    $channel_rename.write().set_text($channel.name().to_owned());

    watch!($channel.name().to_owned())
      .distinct_until_changed()
      .subscribe(move |name| {
        $channel_rename.write().set_text(name);
      });

    let app_writer_cancel_ref = app.clone_writer();
    let app_writer_confirm_ref = app.clone_writer();

    @Modal {
      title: "Channel Settings",
      size: Size::new(480., 530.),
      confirm_cb: Box::new(move || {
        app_writer_confirm_ref.write().set_modify_channel_id(None);
      }) as Box<dyn Fn()>,
      cancel_cb: Box::new(move || {
        app_writer_cancel_ref.write().set_modify_channel_id(None);
      }) as Box<dyn Fn()>,
      @Stack {
        @Column {
          @Text {
            text: "Channel Name",
            text_style: TypographyTheme::of(ctx!()).title_medium.text.clone(),
          }
          @$channel_rename {
            cursor: CursorIcon::Text,
            h_align: HAlign::Stretch,
            margin: EdgeInsets::only_bottom(10.),
            background: Color::from_u32(CULTURED_F7F7F5_FF),
            padding: EdgeInsets::new(10., 5., 10., 5.),
            border: Border::all(BorderSide {
              width: 1.,
              color: Color::from_u32(CHINESE_WHITE).into(),
            }),
            border_radius: Radius::all(6.),
            @ { Placeholder::new("Type a channel name") }
          }
          @Text {
            text: "Default AI Bot",
            text_style: TypographyTheme::of(ctx!()).title_medium.text.clone(),
          }
          @Text {
            text: "Optimize Performance",
            text_style: TypographyTheme::of(ctx!()).title_medium.text.clone(),
          }
          @Column {
            @$balanced_mode {
              on_tap: move |_| {

              }
            }
            @$performance_mode {
              on_tap: move |_| {

              }
            }
          }
        }
      }
    }
  }
}

impl<T: StateWriter<Value = AppGUI>> AppExtraWidgets for T {}

trait AppExtraWidgets: StateWriter<Value = AppGUI> + Sized {
  // TODO: others data operators.
}

// launch App need to do some init work.
// 1. [x] load bot config file.
// 2. [ ] if has user module, need check user login status.
// 3. [ ] some static file can't be load dynamic, need load as bytes before
//    launch.
// 4. [ ] load local db data.
pub fn w_app() -> impl WidgetBuilder {
  let app_data = mock_init_app_data();

  // XXX: how to build `AppGUI` global state?
  // 1. If I create global state `Stateful<AppGUI>`, `fn_widget!` need capture
  //    this global state ownership. Like this global state is pointless.
  // 2. If I create global state `AppGUI`, `Stateful<&mut AppGUI>` can't compiler.

  fn_widget! { AppGUI::new(app_data) }
}

fn mock_init_app_data() -> AppData {
  let bots = load_bot_cfg_file();
  let first_bot_id = *bots.first().unwrap().id();
  let mut app_data = AppData::new(bots, first_bot_id);
  app_data.clear_channels();
  app_data.new_channel("quick start".to_owned(), None);
  app_data
}
