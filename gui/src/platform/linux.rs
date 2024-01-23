#[cfg(target_os = "linux")]
pub fn app_init_hook() -> bool { super::singleton_guard() }

#[cfg(target_os = "linux")]
pub fn app_run_before_hook() {}

#[cfg(target_os = "linux")]
pub fn permission_prompt() -> bool {
  // mock for linux, wait for implementation
  true
}

#[cfg(target_os = "linux")]
pub fn has_permission() -> bool {
  // mock for linux, wait for implementation
  true
}
