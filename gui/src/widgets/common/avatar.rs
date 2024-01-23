use polestar_core::{get_static_file, model::BotAvatar};
use ribir::prelude::*;

pub fn w_avatar(avatar: BotAvatar) -> impl WidgetBuilder {
  fn_widget! {
    @ {
      match avatar {
        BotAvatar::Image { url } => {
          let data = get_static_file(&url);
          if let Ok(data) = data {
            @Avatar {
              @ { ShareResource::new(PixelImage::from_png(&data)) }
            }.widget_build(ctx!())
          } else {
            @Void {}.widget_build(ctx!())
          }
        }
        BotAvatar::Text { name, color } => {
          @Avatar {
            color: Color::from_u32(get_color_by_hex_str(&color)),
            @ { Label::new(name.to_owned()) }
          }.widget_build(ctx!())
        }
      }
    }
  }
}

fn get_color_by_hex_str(color: &str) -> u32 {
  let color = color.trim_start_matches('#');
  u32::from_str_radix(color, 16).unwrap_or_default()
}
