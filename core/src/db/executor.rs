use uuid::Uuid;

use crate::{
  error::PolestarResult,
  model::{Attachment, Channel, Msg},
};

use super::pool::DbPool;

pub mod attachment;
pub mod channel;
pub mod msg;

#[derive(Clone)]
pub enum ActionPersist {
  AddMsg { channel_id: Uuid, msg: Msg },
  UpdateMsg { msg: Msg },
  AddChannel { channel: Channel },
  RemoveChannel { channel_id: Uuid },
  UpdateChannel { channel: Channel },
  AddAttachment { attachment: Attachment },
}

pub trait Persist {
  fn write(&self, pool: &DbPool) -> impl std::future::Future<Output = PolestarResult<()>>;
}

impl Persist for ActionPersist {
  async fn write(&self, pool: &DbPool) -> PolestarResult<()> {
    match self {
      ActionPersist::AddMsg { channel_id, msg } => {
        msg::add_msg(pool, channel_id, msg).await?;
      }
      ActionPersist::UpdateMsg { msg } => {
        msg::update_msg(pool, msg).await?;
      }
      ActionPersist::AddChannel { channel } => {
        channel::add_channel(pool, channel).await?;
      }
      ActionPersist::RemoveChannel { channel_id } => {
        channel::remove_channel(pool, channel_id).await?;
      }
      ActionPersist::UpdateChannel { channel } => {
        channel::update_channel(pool, channel).await?;
      }
      ActionPersist::AddAttachment { attachment } => {
        attachment::add_attachment(pool, attachment).await?;
      }
    }

    Ok(())
  }
}
