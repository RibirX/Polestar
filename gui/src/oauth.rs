mod apple;
pub use apple::apple_login_uri;
mod google;
pub use google::google_login_uri;
mod microsoft;
pub use microsoft::microsoft_login_uri;

#[cfg(test)]
pub(self) fn generate_state() -> String { String::new() }

#[cfg(not(test))]
pub(self) fn generate_state() -> String {
  use crate::GLOBAL_CONFIG;
  use base64::{engine::general_purpose, Engine as _};
  use std::collections::HashMap;

  let server_local_port = "server_local_port";
  let mut m = HashMap::new();

  if let Some(port) = GLOBAL_CONFIG.lock().unwrap().local_server_port {
    m.insert(server_local_port.to_string(), port.to_string());
  }
  let global_state = serde_json::to_string(&m);
  if let Ok(state) = global_state {
    general_purpose::STANDARD.encode(state.as_bytes())
  } else {
    String::new()
  }
}
