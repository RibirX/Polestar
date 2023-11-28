use std::collections::HashMap;

use ribir::{material::typography_theme, prelude::*};

// TODO: ribir need to support inherit_theme define icon
macro_rules! fill_svgs {
  ($theme: expr, $($name: path: $path: literal),+) => {
    $(
      let icon = ShareResource::new(include_svg!($path));
      $theme.insert($name,  icon);
    )+
  };
}

pub mod polestar_svg {
  use ribir::core::{define_named_svg, prelude::*};
  define_named_svg! {
    CUSTOM_ICON_START,
    LOGO,
    CLIPBOARD,
    USER_AVATAR,
    REPLY,
    APPLE_LOGIN,
    MICROSOFT_LOGIN,
    GOOGLE_LOGIN,
    RETRY,
    CLOSE_GRAY,
    LEFT_ARROW,
    RIGHT_ARROW,
    LOADING,
    EXPAND_LESS,
    ADD_CIRCLE,
    CHECK_CIRCLE,
    CHECK,
    FILE_DOWNLOAD,
    GRADE,
    REFRESH,
    SMS,
    ACCOUNT_CIRCLE,
    INFO,
    KEYBOARD_COMMAND,
    KEYBOARD_RETURN,
    EDIT,
    SEND,
    TRASH,
    MATERIAL_THEME_END
  }
}

pub fn polestar_theme() -> InheritTheme {
  let regular_family = Box::new([
    FontFamily::Name("Inter".into()),
    FontFamily::Name("PingFang SC".into()),
    FontFamily::Name("Segoe UI".into()),
    FontFamily::Name("Helvetica".into()),
    FontFamily::Name("Arial".into()),
    FontFamily::Name("Noto Sans SC".into()),
    FontFamily::Name("Noto Color Emoji".into()),
  ]);
  let medium_family = Box::new([
    FontFamily::Name("Inter".into()),
    FontFamily::Name("PingFang SC".into()),
    FontFamily::Name("Segoe UI".into()),
    FontFamily::Name("Helvetica".into()),
    FontFamily::Name("Arial".into()),
    FontFamily::Name("Noto Sans SC".into()),
    FontFamily::Name("Noto Color Emoji".into()),
  ]);
  let typography_theme = typography_theme(
    regular_family,
    medium_family,
    TextDecoration::NONE,
    Color::BLACK.with_alpha(0.87).into(),
  );
  let mut inherit_theme = InheritTheme {
    typography_theme: Some(typography_theme),
    font_bytes: Some(vec![
      include_bytes!("./theme/fonts/Inter-Regular.otf").to_vec(),
      include_bytes!("./theme/fonts/Inter-Bold.otf").to_vec(),
      include_bytes!("./theme/fonts/Inter-Medium.otf").to_vec(),
      include_bytes!("./theme/fonts/NotoSansSC-Regular.otf").to_vec(),
      include_bytes!("./theme/fonts/NotoSansSC-Bold.otf").to_vec(),
      include_bytes!("./theme/fonts/NotoSansSC-Medium.otf").to_vec(),
      include_bytes!("./theme/fonts/NotoColorEmoji.ttf").to_vec(),
    ]),
    ..<_>::default()
  };

  let mut icons = HashMap::<NamedSvg, ShareResource<Svg>, ahash::RandomState>::default();

  // XXX: will override default theme svg icon.
  fill_svgs! {
    icons,
    svgs::CHECK_BOX: "./theme/icons/radio_button_checked_FILL0_wght400_GRAD0_opsz48.svg",
    svgs::CHECK_BOX_OUTLINE_BLANK: "./theme/icons/radio_button_unchecked_FILL0_wght400_GRAD0_opsz48.svg"
  }

  fill_svgs! {
    icons,
    polestar_svg::LOGO: "./theme/icons/logo.svg",
    polestar_svg::CLIPBOARD: "./theme/icons/clipboard.svg",
    polestar_svg::USER_AVATAR: "./theme/icons/user_avatar.svg",
    polestar_svg::REPLY: "./theme/icons/reply.svg",
    polestar_svg::APPLE_LOGIN: "./theme/icons/apple_logo_mark.svg",
    polestar_svg::MICROSOFT_LOGIN: "./theme/icons/google_logo_mark.svg",
    polestar_svg::GOOGLE_LOGIN: "./theme/icons/microsoft_logo_mark.svg",
    polestar_svg::RETRY: "./theme/icons/retry.svg",
    polestar_svg::CLOSE_GRAY: "./theme/icons/close_gray.svg",
    polestar_svg::LEFT_ARROW: "./theme/icons/left_arrow.svg",
    polestar_svg::RIGHT_ARROW: "./theme/icons/right_arrow.svg",
    polestar_svg::LOADING: "./theme/icons/loading.svg",
    polestar_svg::EXPAND_LESS: "./theme/icons/expand_less_FILL0_wght400_GRAD0_opsz48.svg",
    polestar_svg::ADD_CIRCLE: "./theme/icons/add_circle_FILL0_wght400_GRAD0_opsz48.svg",
    polestar_svg::CHECK_CIRCLE: "./theme/icons/check_circle_FILL0_wght400_GRAD0_opsz48.svg",
    polestar_svg::CHECK: "./theme/icons/check_FILL0_wght400_GRAD0_opsz48.svg",
    polestar_svg::FILE_DOWNLOAD: "./theme/icons/file_download_FILL0_wght400_GRAD0_opsz48.svg",
    polestar_svg::GRADE: "./theme/icons/grade_FILL0_wght400_GRAD0_opsz48.svg",
    polestar_svg::REFRESH: "./theme/icons/refresh_FILL0_wght400_GRAD0_opsz48.svg",
    polestar_svg::SMS: "./theme/icons/sms_FILL0_wght400_GRAD0_opsz48.svg",
    polestar_svg::ACCOUNT_CIRCLE: "./theme/icons/account_circle_FILL0_wght400_GRAD0_opsz48.svg",
    polestar_svg::INFO: "./theme/icons/info_FILL0_wght400_GRAD0_opsz48.svg",
    polestar_svg::KEYBOARD_COMMAND: "./theme/icons/keyboard_command_key_FILL0_wght400_GRAD0_opsz48.svg",
    polestar_svg::KEYBOARD_RETURN: "./theme/icons/keyboard_return_FILL0_wght400_GRAD0_opsz48.svg",
    polestar_svg::EDIT: "./theme/icons/pencil.svg",
    polestar_svg::SEND: "./theme/icons/send.svg",
    polestar_svg::TRASH: "./theme/icons/trash.svg"
  }

  inherit_theme.icons = Some(icons);

  inherit_theme
}
