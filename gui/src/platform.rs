#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::{app_init_hook, app_run_before_hook, has_permission, permission_prompt};

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::{app_init_hook, app_run_before_hook, has_permission, permission_prompt};
