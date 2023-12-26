use uuid::Uuid;

use crate::{
  error::PolestarResult,
  model::{Attachment, ChannelCfg, ChannelId, Msg},
};

use super::pool::DbPool;

pub mod attachment;
pub mod channel;
pub mod msg;

#[derive(Clone)]
pub enum ActionPersist {
  AddMsg {
    channel_id: Uuid,
    msg: Msg,
  },
  UpdateMsg {
    msg: Msg,
  },
  RemoveMsg {
    msg_id: Uuid,
  },
  AddChannel {
    id: ChannelId,
    name: String,
    desc: Option<String>,
    cfg: ChannelCfg,
  },
  RemoveChannel {
    channel_id: Uuid,
  },
  UpdateChannel {
    id: ChannelId,
    name: String,
    desc: Option<String>,
    cfg: ChannelCfg,
  },
  AddAttachment {
    attachment: Attachment,
  },
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
      ActionPersist::RemoveMsg { msg_id } => {
        msg::remove_msg(pool, msg_id).await?;
      }
      ActionPersist::AddChannel { id, name, desc, cfg } => {
        channel::add_channel(pool, id, name.as_str(), desc.as_deref(), cfg).await?;
      }
      ActionPersist::RemoveChannel { channel_id } => {
        channel::remove_channel(pool, channel_id).await?;
      }
      ActionPersist::UpdateChannel { id, name, desc, cfg } => {
        channel::update_channel(pool, id, name, desc.as_deref(), cfg).await?;
      }
      ActionPersist::AddAttachment { attachment } => {
        attachment::add_attachment(pool, attachment).await?;
      }
    }

    Ok(())
  }
}
