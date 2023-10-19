use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, sqlx::Type)]
pub enum MIME {
  #[serde(rename = "image/png")]
  ImagePng,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Attachment {
  name: Uuid,
  mime: MIME,
  data: Vec<u8>,
}

impl Attachment {
  pub fn new(media_type: MIME, data: Vec<u8>) -> Self {
    Self {
      name: Uuid::new_v4(),
      mime: media_type,
      data,
    }
  }

  pub fn name(&self) -> &Uuid { &self.name }

  pub fn data(&self) -> &[u8] { &self.data }

  pub fn mime(&self) -> &MIME { &self.mime }
}
