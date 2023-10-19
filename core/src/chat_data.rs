use tokio::runtime::Runtime;
use uuid::Uuid;

use crate::{
  db::{
    self,
    pool::{db_path, DbPool},
  },
  error::PolestarError,
  model::{attachment::Attachment, channel::Channel, msg::Msg},
};

#[derive(Default)]
pub struct ChatData<T: ChatDataOperator> {
  channels: Vec<Channel>,
  cur_channel_id: Uuid,
  msgs: Vec<Msg>,
  store: T,
}

pub struct DbBlockingClient {
  inner: DbPool,
  rt: Runtime,
}

impl DbBlockingClient {
  pub fn connect(db_path: &str) -> Result<Self, PolestarError> {
    let rt = tokio::runtime::Builder::new_current_thread()
      .enable_all()
      .build()?;
    let inner = rt.block_on(db::pool::db_pool(db_path))?;
    Ok(Self { inner, rt })
  }
}

impl Default for DbBlockingClient {
  fn default() -> Self { DbBlockingClient::connect(&db_path()).expect("connect to db failed") }
}

impl ChatDataOperator for DbBlockingClient {
  fn add_msg(&self, channel_id: &Uuid, msg: &Msg) -> Result<(), PolestarError> {
    self
      .rt
      .block_on(db::executor::msg::add_msg(&self.inner, channel_id, msg))
  }

  fn query_msg_by_id(&self, id: &Uuid) -> Result<Msg, PolestarError> {
    self
      .rt
      .block_on(db::executor::msg::query_msg_by_id(&self.inner, id))
  }

  fn update_msg(&self, msg: &Msg) -> Result<(), PolestarError> {
    self
      .rt
      .block_on(db::executor::msg::update_msg(&self.inner, msg))
  }

  fn query_msgs_by_channel_id(&self, id: &Uuid) -> Result<Vec<Msg>, PolestarError> {
    self
      .rt
      .block_on(db::executor::msg::query_msgs_by_channel_id(&self.inner, id))
  }

  fn query_latest_text_msgs_by_channel_id(
    &self,
    id: &Uuid,
    limit: u32,
  ) -> Result<Vec<Msg>, PolestarError> {
    self
      .rt
      .block_on(db::executor::msg::query_latest_text_msgs_by_channel_id(
        &self.inner,
        id,
        limit,
      ))
  }

  fn add_channel(&self, channel: &Channel) -> Result<(), PolestarError> {
    self
      .rt
      .block_on(db::executor::channel::add_channel(&self.inner, channel))
  }

  fn remove_channel(&self, id: &Uuid) -> Result<(), PolestarError> {
    self
      .rt
      .block_on(db::executor::channel::remove_channel(&self.inner, id))
  }

  fn query_channel_by_id(&self, id: &Uuid) -> Result<Channel, PolestarError> {
    self
      .rt
      .block_on(db::executor::channel::query_channel_by_id(&self.inner, id))
  }

  fn update_channel(&self, channel: &Channel) -> Result<(), PolestarError> {
    self
      .rt
      .block_on(db::executor::channel::update_channel(&self.inner, channel))
  }

  fn query_channels(&self) -> Result<Vec<Channel>, PolestarError> {
    self
      .rt
      .block_on(db::executor::channel::query_channels(&self.inner))
  }

  fn add_attachment(&self, attachment: &Attachment) -> Result<Uuid, PolestarError> {
    self.rt.block_on(db::executor::attachment::add_attachment(
      &self.inner,
      attachment,
    ))
  }

  fn query_attachment_by_name(&self, name: &Uuid) -> Result<Attachment, PolestarError> {
    self
      .rt
      .block_on(db::executor::attachment::query_attachment_by_name(
        &self.inner,
        name,
      ))
  }
}

impl<T: ChatDataOperator> ChatData<T> {
  pub fn set_store(&mut self, store: T) { self.store = store; }

  pub fn switch_channel(&mut self, id: &Uuid) -> Result<(), PolestarError> {
    let idx = self.channels.iter().position(|channel| channel.id() == id);

    if let Some(idx) = idx {
      self.cur_channel_id = *self.channels[idx].id();
    } else {
      log::warn!(
        "[polestar] switch channel error: channel id {} not found",
        id
      );
    }

    self.msgs.clear();

    match self.store.query_msgs_by_channel_id(id) {
      Ok(msgs) => {
        self.msgs = msgs;
        Ok(())
      }
      Err(err) => {
        log::error!("[polestar] switch channel error: {}", err);
        Err(err)
      }
    }
  }

  pub fn add_channel(&mut self, channel: Channel) -> Result<(), PolestarError> {
    let id = channel.id();

    if self.channels.iter().any(|channel| channel.id() == id) {
      log::warn!(
        "[polestar] add channel error: channel id {} already exists",
        id
      );
      return Ok(());
    }

    if let Err(err) = self.store.add_channel(&channel) {
      log::error!("[polestar] add channel error: {}", err);
      return Err(err);
    }

    self.cur_channel_id = *id;
    self.channels.push(channel);

    Ok(())
  }

  pub fn remove_channel(&mut self, id: &Uuid) -> Result<(), PolestarError> {
    let idx = self.channels.iter().position(|channel| channel.id() == id);

    if let Some(idx) = idx {
      if let Err(err) = self.store.remove_channel(id) {
        log::error!("[polestar] remove channel error: {}", err);
        return Err(err);
      }

      self.channels.remove(idx);

      if id == &self.cur_channel_id {
        if self.channels.is_empty() {
          self.cur_channel_id = Uuid::nil();
        } else if idx == 0 {
          self.cur_channel_id = *self.channels[idx].id();
        } else {
          self.cur_channel_id = *self.channels[idx - 1].id();
        }
      }
    } else {
      log::warn!(
        "[polestar] remove channel error: channel id {} not found",
        id
      );
    }

    Ok(())
  }

  pub fn add_msg(&mut self, msg: Msg) -> Result<(), PolestarError> {
    if let Err(err) = self.store.add_msg(&self.cur_channel_id, &msg) {
      log::error!("[polestar] add msg error: {}", err);
      return Err(err);
    }
    self.msgs.push(msg);
    Ok(())
  }

  pub fn cur_channel_id(&self) -> &Uuid { &self.cur_channel_id }

  pub fn channels(&self) -> &Vec<Channel> { &self.channels }

  pub fn msgs(&self) -> &Vec<Msg> { &self.msgs }

  #[allow(unused)]
  fn add_attachment(&mut self, attachment: Attachment) -> Result<Uuid, PolestarError> {
    match self.store.add_attachment(&attachment) {
      Ok(name) => Ok(name),
      Err(err) => {
        log::error!("[polestar] add attachment error: {}", err);
        Err(err)
      }
    }
  }

  #[allow(unused)]
  fn query_attachment_by_name(&self, name: &Uuid) -> Result<Attachment, PolestarError> {
    match self.store.query_attachment_by_name(name) {
      Ok(attachment) => Ok(attachment),
      Err(err) => {
        log::error!("[polestar] query attachment error: {}", err);
        Err(err)
      }
    }
  }
}

pub trait ChatDataOperator: Default {
  fn add_msg(&self, channel_id: &Uuid, msg: &Msg) -> Result<(), PolestarError>;
  fn query_msg_by_id(&self, id: &Uuid) -> Result<Msg, PolestarError>;
  fn update_msg(&self, msg: &Msg) -> Result<(), PolestarError>;
  fn query_msgs_by_channel_id(&self, id: &Uuid) -> Result<Vec<Msg>, PolestarError>;
  fn query_latest_text_msgs_by_channel_id(
    &self,
    id: &Uuid,
    limit: u32,
  ) -> Result<Vec<Msg>, PolestarError>;
  fn add_channel(&self, channel: &Channel) -> Result<(), PolestarError>;
  fn remove_channel(&self, id: &Uuid) -> Result<(), PolestarError>;
  fn query_channel_by_id(&self, id: &Uuid) -> Result<Channel, PolestarError>;
  fn update_channel(&self, channel: &Channel) -> Result<(), PolestarError>;
  fn query_channels(&self) -> Result<Vec<Channel>, PolestarError>;
  fn add_attachment(&self, attachment: &Attachment) -> Result<Uuid, PolestarError>;
  fn query_attachment_by_name(&self, name: &Uuid) -> Result<Attachment, PolestarError>;
}
