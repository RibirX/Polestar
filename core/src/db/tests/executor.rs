use std::{ptr::NonNull, thread::sleep, time::Duration};

use uuid::Uuid;

use crate::{
  db::{executor::ActionPersist, pool::PersistenceDB},
  model::{Attachment, Channel, ChannelCfg, Msg, MsgCont, MsgMeta, MsgRole, MIME},
};

use super::common::init_db;

fn log_init() { let _ = env_logger::builder().is_test(true).try_init(); }

#[test]
fn add_channel_test() {
  log_init();

  let persistence_db =
    PersistenceDB::connect(init_db(), Duration::from_millis(100)).expect("Failed to connect db");

  let mut persistence_db = Box::pin(persistence_db);

  let id_1 = Uuid::new_v4();
  let channel_1 = Channel::new(
    id_1,
    "test".to_owned(),
    Some("test channel".to_owned()),
    ChannelCfg::default(),
    None,
    Some(NonNull::from(&*persistence_db)),
  );
  persistence_db
    .as_mut()
    .pin_add_persist(ActionPersist::AddChannel { channel: channel_1 });
  let id_2 = Uuid::new_v4();
  let channel_2 = Channel::new(
    id_2,
    "test 2".to_owned(),
    Some("test channel 2".to_owned()),
    ChannelCfg::default(),
    None,
    Some(NonNull::from(&*persistence_db)),
  );
  persistence_db
    .as_mut()
    .pin_add_persist(ActionPersist::AddChannel { channel: channel_2 });

  sleep(Duration::from_millis(100));

  let channels = persistence_db
    .query_channels()
    .expect("Failed to query channels");
  assert_eq!(channels.len(), 2);
}

#[test]
fn remove_channel_test() {
  log_init();

  let persistence_db =
    PersistenceDB::connect(init_db(), Duration::from_millis(100)).expect("Failed to connect db");

  let mut persistence_db = Box::pin(persistence_db);

  let id_1 = Uuid::new_v4();
  let channel_1 = Channel::new(
    id_1,
    "test".to_owned(),
    Some("test channel".to_owned()),
    ChannelCfg::default(),
    None,
    Some(NonNull::from(&*persistence_db)),
  );
  persistence_db
    .as_mut()
    .pin_add_persist(ActionPersist::AddChannel { channel: channel_1 });
  let id_2 = Uuid::new_v4();
  let channel_2 = Channel::new(
    id_2,
    "test 2".to_owned(),
    Some("test channel 2".to_owned()),
    ChannelCfg::default(),
    None,
    Some(NonNull::from(&*persistence_db)),
  );
  persistence_db
    .as_mut()
    .pin_add_persist(ActionPersist::AddChannel { channel: channel_2 });

  sleep(Duration::from_millis(100));

  let channels = persistence_db
    .query_channels()
    .expect("Failed to query channels");
  assert_eq!(channels.len(), 2);

  persistence_db
    .as_mut()
    .pin_add_persist(ActionPersist::RemoveChannel { channel_id: id_1 });

  sleep(Duration::from_millis(100));

  let channels = persistence_db
    .query_channels()
    .expect("Failed to query channels");
  assert_eq!(channels.len(), 1);
}

#[test]
fn update_channel_test() {
  log_init();

  let persistence_db =
    PersistenceDB::connect(init_db(), Duration::from_millis(100)).expect("Failed to connect db");

  let mut persistence_db = Box::pin(persistence_db);

  let id_1 = Uuid::new_v4();
  let channel_1 = Channel::new(
    id_1,
    "test".to_owned(),
    Some("test channel".to_owned()),
    ChannelCfg::default(),
    None,
    Some(NonNull::from(&*persistence_db)),
  );
  persistence_db
    .as_mut()
    .pin_add_persist(ActionPersist::AddChannel { channel: channel_1 });

  sleep(Duration::from_millis(100));

  let channels = persistence_db
    .query_channels()
    .expect("Failed to query channels");
  assert_eq!(channels.len(), 1);

  let mut channel_1 = channels[0].clone();
  channel_1.set_name("test 2".to_owned());
  persistence_db
    .as_mut()
    .pin_add_persist(ActionPersist::UpdateChannel { channel: channel_1 });

  sleep(Duration::from_millis(100));

  let channels = persistence_db
    .query_channels()
    .expect("Failed to query channels");
  assert_eq!(channels.len(), 1);
  assert_eq!(channels[0].name(), "test 2");

  let mut channel_1 = channels[0].clone();
  channel_1.set_desc(Some("test channel 2".to_owned()));
  persistence_db
    .as_mut()
    .pin_add_persist(ActionPersist::UpdateChannel { channel: channel_1 });

  sleep(Duration::from_millis(100));

  let channels = persistence_db
    .query_channels()
    .expect("Failed to query channels");
  assert_eq!(channels.len(), 1);
  assert_eq!(channels[0].desc(), Some("test channel 2"));
}

#[test]
fn add_msg_test() {
  log_init();

  let persistence_db =
    PersistenceDB::connect(init_db(), Duration::from_millis(100)).expect("Failed to connect db");
  let mut persistence_db = Box::pin(persistence_db);

  let channel_id = Uuid::new_v4();
  let channel = Channel::new(
    channel_id,
    "test".to_owned(),
    Some("test channel".to_owned()),
    ChannelCfg::default(),
    None,
    Some(NonNull::from(&*persistence_db)),
  );
  persistence_db
    .as_mut()
    .pin_add_persist(ActionPersist::AddChannel { channel });

  sleep(Duration::from_millis(100));

  let channels = persistence_db
    .query_channels()
    .expect("Failed to query channels");
  assert_eq!(channels.len(), 1);

  let channel = &channels[0];

  let msg = Msg::new(
    MsgRole::User,
    vec![MsgCont::init_text()],
    MsgMeta::default(),
  );
  persistence_db
    .as_mut()
    .pin_add_persist(ActionPersist::AddMsg { channel_id, msg });

  sleep(Duration::from_millis(100));

  let msgs = persistence_db
    .query_msgs_by_channel_id(channel.id())
    .expect("Failed to query msgs");
  assert_eq!(msgs.len(), 1);
}

#[test]
fn update_msg_test() {
  log_init();

  let persistence_db =
    PersistenceDB::connect(init_db(), Duration::from_millis(100)).expect("Failed to connect db");

  let mut persistence_db = Box::pin(persistence_db);

  let channel_id = Uuid::new_v4();
  let channel = Channel::new(
    channel_id,
    "test".to_owned(),
    Some("test channel".to_owned()),
    ChannelCfg::default(),
    None,
    Some(NonNull::from(&*persistence_db)),
  );
  persistence_db
    .as_mut()
    .pin_add_persist(ActionPersist::AddChannel { channel });

  sleep(Duration::from_millis(100));

  let channels = persistence_db
    .query_channels()
    .expect("Failed to query channels");
  assert_eq!(channels.len(), 1);

  let channel = &channels[0];

  let msg = Msg::new(
    MsgRole::User,
    vec![MsgCont::init_text()],
    MsgMeta::default(),
  );
  persistence_db
    .as_mut()
    .pin_add_persist(ActionPersist::AddMsg { channel_id, msg });

  sleep(Duration::from_millis(100));

  let msgs = persistence_db
    .query_msgs_by_channel_id(channel.id())
    .expect("Failed to query msgs");
  assert_eq!(msgs.len(), 1);

  let mut msg = msgs[0].clone();
  msg.add_cont(MsgCont::init_text());
  persistence_db
    .as_mut()
    .pin_add_persist(ActionPersist::UpdateMsg { msg });

  sleep(Duration::from_millis(100));

  let msgs = persistence_db
    .query_msgs_by_channel_id(channel.id())
    .expect("Failed to query msgs");
  assert_eq!(msgs.len(), 1);
  assert_eq!(msgs[0].cur_idx(), 1);
  assert_eq!(msgs[0].cont_count(), 2);
}

#[test]
fn query_msgs_by_channel_id_test() {
  log_init();

  let persistence_db =
    PersistenceDB::connect(init_db(), Duration::from_millis(100)).expect("Failed to connect db");
  let mut persistence_db = Box::pin(persistence_db);

  let channel_id = Uuid::new_v4();
  let channel = Channel::new(
    channel_id,
    "test".to_owned(),
    Some("test channel".to_owned()),
    ChannelCfg::default(),
    None,
    Some(NonNull::from(&*persistence_db)),
  );
  persistence_db
    .as_mut()
    .pin_add_persist(ActionPersist::AddChannel { channel });

  sleep(Duration::from_millis(100));

  let channels = persistence_db
    .query_channels()
    .expect("Failed to query channels");
  assert_eq!(channels.len(), 1);

  let channel = &channels[0];

  let msg = Msg::new(
    MsgRole::User,
    vec![MsgCont::init_text()],
    MsgMeta::default(),
  );
  persistence_db
    .as_mut()
    .pin_add_persist(ActionPersist::AddMsg { channel_id, msg });

  sleep(Duration::from_millis(100));

  let msgs = persistence_db
    .query_msgs_by_channel_id(channel.id())
    .expect("Failed to query msgs");
  assert_eq!(msgs.len(), 1);
}

#[test]
fn add_attachment_test() {
  log_init();

  let persistence_db =
    PersistenceDB::connect(init_db(), Duration::from_millis(100)).expect("Failed to connect db");
  let mut persistence_db = Box::pin(persistence_db);

  let attachment = Attachment::new(MIME::ImagePng, vec![1, 2, 3, 4]);
  let attachment_clone = attachment.clone();
  persistence_db
    .as_mut()
    .pin_add_persist(ActionPersist::AddAttachment { attachment });

  sleep(Duration::from_millis(100));

  let query_attachment = persistence_db
    .query_attachment_by_name(attachment_clone.name())
    .expect("Failed to query attachment");

  assert_eq!(attachment_clone.name(), query_attachment.name());
  assert_eq!(attachment_clone.mime(), &MIME::ImagePng);
  assert_eq!(attachment_clone.data(), &[1, 2, 3, 4]);
}
