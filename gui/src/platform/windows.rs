#[cfg(target_os = "windows")]
pub fn app_init_hook() -> bool { super::singleton_guard() }

#[cfg(target_os = "windows")]
pub fn app_run_before_hook() {}

#[cfg(target_os = "windows")]
pub fn permission_prompt() -> bool {
  // mock for windows, wait for windows implementation
  true
}

#[cfg(target_os = "windows")]
pub fn has_permission() -> bool {
  // mock for windows, wait for windows implementation
  true
}
