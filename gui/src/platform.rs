#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::{app_init_hook, app_run_before_hook, has_permission, permission_prompt};

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::{app_init_hook, app_run_before_hook, has_permission, permission_prompt};

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::{app_init_hook, app_run_before_hook, has_permission, permission_prompt};

fn singleton_guard() -> bool {
  use fs4::FileExt;
  use once_cell::sync::Lazy;
  use polestar_core::create_if_not_exist_dir;
  use polestar_core::project_home_path;
  use ribir::prelude::log::warn;
  use std::fs::OpenOptions;

  create_if_not_exist_dir(project_home_path());
  static GUARD_FILE: Lazy<(Result<std::fs::File, std::io::Error>, bool)> = Lazy::new(|| {
    let file_path = format!("{}/{}", project_home_path().display(), "/singleton_guard");
    let file = OpenOptions::new().create(true).write(true).open(file_path);
    let singleton = match file {
      Ok(ref f) => {
        let lock = f.try_lock_exclusive();
        lock.is_ok()
      }
      Err(ref e) => {
        warn!("Failed to open singleton_guard file: {}", e);
        false
      }
    };
    (file, singleton)
  });
  GUARD_FILE.1
}
