use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct LocalState {
  cur_channel_id: Option<Uuid>,
  quick_launcher_id: Option<Uuid>,
  uid: Option<u64>,
}

impl LocalState {
  pub fn new(
    cur_channel_id: Option<Uuid>,
    quick_launcher_id: Option<Uuid>,
    uid: Option<u64>,
  ) -> Self {
    Self {
      cur_channel_id,
      quick_launcher_id,
      uid,
    }
  }

  pub fn cur_channel_id(&self) -> &Option<Uuid> { &self.cur_channel_id }

  pub fn quick_launcher_id(&self) -> &Option<Uuid> { &self.quick_launcher_id }

  pub fn uid(&self) -> &Option<u64> { &self.uid }
}
