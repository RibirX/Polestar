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

pub mod polestar_svgs {
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

pub fn theme() -> InheritTheme {
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

  fill_svgs! {
    icons,
    polestar_svgs::LOGO: "./theme/icons/logo.svg",
    polestar_svgs::CLIPBOARD: "./theme/icons/clipboard.svg",
    polestar_svgs::USER_AVATAR: "./theme/icons/user_avatar.svg",
    polestar_svgs::REPLY: "./theme/icons/reply.svg",
    polestar_svgs::APPLE_LOGIN: "./theme/icons/apple_logo_mark.svg",
    polestar_svgs::MICROSOFT_LOGIN: "./theme/icons/google_logo_mark.svg",
    polestar_svgs::GOOGLE_LOGIN: "./theme/icons/microsoft_logo_mark.svg",
    polestar_svgs::RETRY: "./theme/icons/retry.svg",
    polestar_svgs::CLOSE_GRAY: "./theme/icons/close_gray.svg",
    polestar_svgs::LEFT_ARROW: "./theme/icons/left_arrow.svg",
    polestar_svgs::RIGHT_ARROW: "./theme/icons/right_arrow.svg",
    polestar_svgs::LOADING: "./theme/icons/loading.svg",
    polestar_svgs::EXPAND_LESS: "./theme/icons/expand_less_FILL0_wght400_GRAD0_opsz48.svg",
    polestar_svgs::ADD_CIRCLE: "./theme/icons/add_circle_FILL0_wght400_GRAD0_opsz48.svg",
    polestar_svgs::CHECK_CIRCLE: "./theme/icons/check_circle_FILL0_wght400_GRAD0_opsz48.svg",
    polestar_svgs::CHECK: "./theme/icons/check_FILL0_wght400_GRAD0_opsz48.svg",
    polestar_svgs::FILE_DOWNLOAD: "./theme/icons/file_download_FILL0_wght400_GRAD0_opsz48.svg",
    polestar_svgs::GRADE: "./theme/icons/grade_FILL0_wght400_GRAD0_opsz48.svg",
    polestar_svgs::REFRESH: "./theme/icons/refresh_FILL0_wght400_GRAD0_opsz48.svg",
    polestar_svgs::SMS: "./theme/icons/sms_FILL0_wght400_GRAD0_opsz48.svg",
    polestar_svgs::ACCOUNT_CIRCLE: "./theme/icons/account_circle_FILL0_wght400_GRAD0_opsz48.svg",
    polestar_svgs::INFO: "./theme/icons/info_FILL0_wght400_GRAD0_opsz48.svg",
    polestar_svgs::KEYBOARD_COMMAND: "./theme/icons/keyboard_command_key_FILL0_wght400_GRAD0_opsz48.svg",
    polestar_svgs::KEYBOARD_RETURN: "./theme/icons/keyboard_return_FILL0_wght400_GRAD0_opsz48.svg",
    polestar_svgs::EDIT: "./theme/icons/pencil.svg",
    polestar_svgs::SEND: "./theme/icons/send.svg",
    polestar_svgs::TRASH: "./theme/icons/trash.svg"
  }

  inherit_theme.icons = Some(icons);

  inherit_theme
}
