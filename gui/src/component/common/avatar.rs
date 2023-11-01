use polestar_core::model::BotAvatar;
use ribir::prelude::*;

#[derive(Declare)]
pub struct AvatarWidget {
  avatar: BotAvatar,
}

impl Compose for AvatarWidget {
  fn compose(this: impl StateWriter<Value = Self>) -> impl WidgetBuilder {
    fn_widget! {
      @ {
        match &$this.avatar {
          BotAvatar::Image { url } => {
            let data = vec![0];
            @Avatar {
              @ { ShareResource::new(PixelImage::from_png(&data)) }
            }.widget_build(ctx!())
          }
          BotAvatar::Text { name, color } => {
            @Avatar {
              color: Color::from_u32(get_color_by_hex_str(color)),
              @ { Label::new(name.to_owned()) }
            }.widget_build(ctx!())
          }
        }
      }
    }
  }
}

fn get_color_by_hex_str(color: &str) -> u32 {
  let color = color.trim_start_matches('#');
  u32::from_str_radix(color, 16).unwrap_or_default()
}
