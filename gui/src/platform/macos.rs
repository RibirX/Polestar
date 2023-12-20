#[cfg(target_os = "macos")]
pub fn app_init_hook() -> bool { true }

#[cfg(target_os = "macos")]
pub fn app_run_before_hook() {}

#[cfg(target_os = "macos")]
pub fn permission_prompt() -> bool {
  use macos_accessibility_client::accessibility;
  accessibility::application_is_trusted_with_prompt()
}

#[cfg(target_os = "macos")]
pub fn has_permission() -> bool {
  use macos_accessibility_client::accessibility;
  accessibility::application_is_trusted()
}
