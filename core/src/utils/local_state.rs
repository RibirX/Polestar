use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::write_local_state;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct LocalState {
  cur_channel_id: Option<Uuid>,
  quick_launcher_id: Option<Uuid>,
  uid: Option<u64>,
}

impl LocalState {
  fn save_local(&self) {
    if let Some(uid) = self.uid {
      write_local_state(&uid.to_string(), self).expect("Failed to save local state");
    }
  }

  pub fn cur_channel_id(&self) -> &Option<Uuid> { &self.cur_channel_id }

  pub fn set_cur_channel_id(&mut self, cur_channel_id: Option<Uuid>) {
    self.cur_channel_id = cur_channel_id;
    self.save_local();
  }

  pub fn quick_launcher_id(&self) -> &Option<Uuid> { &self.quick_launcher_id }

  pub fn set_quick_launcher_id(&mut self, quick_launcher_id: Option<Uuid>) {
    self.quick_launcher_id = quick_launcher_id;
    self.save_local();
  }

  pub fn uid(&self) -> &Option<u64> { &self.uid }

  pub fn set_uid(&mut self, uid: Option<u64>) {
    self.uid = uid;
    self.save_local();
  }
}
