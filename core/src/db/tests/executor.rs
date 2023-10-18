use crate::{
  db::executor::{
    channel::{add_channel, query_channel_by_id, update_channel},
    msg::{add_msg, query_msg_by_id, query_msgs_by_channel_id, update_msg},
  },
  model::{
    channel::ChannelBuilder,
    msg::{Msg, MsgCont, MsgMeta, MsgRole},
  },
};

use super::common::init_db;

#[tokio::test]
async fn test_add_channel() {
  let pool = init_db().await.expect("Failed to init database");
  let channel = ChannelBuilder::default()
    .name("test")
    .desc("test channel")
    .build()
    .unwrap();

  add_channel(&pool, &channel)
    .await
    .expect("Failed to add channel");

  let query_channel = query_channel_by_id(&pool, channel.id())
    .await
    .expect("Failed to query channel");

  assert_eq!(channel.id(), query_channel.id());
  assert_eq!(channel.name(), "test");
  assert_eq!(channel.desc(), Some("test channel"));
}

#[tokio::test]
async fn test_remove_channel() {
  let pool = init_db().await.expect("Failed to init database");
  let channel = ChannelBuilder::default()
    .name("test")
    .desc("test channel")
    .build()
    .unwrap();

  add_channel(&pool, &channel)
    .await
    .expect("Failed to add channel");

  let query_channel = query_channel_by_id(&pool, channel.id())
    .await
    .expect("Failed to query channel");

  assert_eq!(channel.id(), query_channel.id());
  assert_eq!(channel.name(), "test");
  assert_eq!(channel.desc(), Some("test channel"));

  let res = sqlx::query(
    r#"
    DELETE FROM channel
    WHERE id = ?1
    "#,
  )
  .bind(channel.id())
  .execute(&pool)
  .await
  .expect("Failed to remove channel");

  assert_eq!(res.rows_affected(), 1);

  let query_channel = query_channel_by_id(&pool, channel.id())
    .await
    .expect_err("Failed to query channel");

  assert_eq!(
    query_channel.to_string(),
    "sqlite error: no rows returned by a query that expected to return at least one row"
  );
}

#[tokio::test]
async fn test_update_channel() {
  let pool = init_db().await.expect("Failed to init database");
  let mut channel = ChannelBuilder::default()
    .name("test")
    .desc("test channel")
    .build()
    .unwrap();

  add_channel(&pool, &channel)
    .await
    .expect("Failed to add channel");

  channel.set_name("test 2".to_owned());

  update_channel(&pool, &channel)
    .await
    .expect("Failed to update channel");

  let query_channel = query_channel_by_id(&pool, channel.id())
    .await
    .expect("Failed to query channel");

  assert_eq!(channel.id(), query_channel.id());
  assert_eq!(channel.name(), "test 2");

  channel.set_desc(Some("test channel 2".to_owned()));

  update_channel(&pool, &channel)
    .await
    .expect("Failed to update channel");

  let query_channel = query_channel_by_id(&pool, channel.id())
    .await
    .expect("Failed to query channel");

  assert_eq!(channel.id(), query_channel.id());
  assert_eq!(channel.desc(), Some("test channel 2"));
}

#[tokio::test]
async fn test_add_msg() {
  let pool = init_db().await.expect("Failed to init database");
  let channel = ChannelBuilder::default()
    .name("test")
    .desc("test channel")
    .build()
    .unwrap();

  let msg = Msg::new(MsgRole::User, MsgCont::text_init(), MsgMeta::default());

  add_msg(&pool, channel.id(), &msg)
    .await
    .expect("Failed to add msg");

  let query_msg = query_msg_by_id(&pool, msg.id())
    .await
    .expect("Failed to query msg");

  assert_eq!(msg.id(), query_msg.id());
  assert_eq!(msg.role(), &MsgRole::User);
  assert_eq!(msg.meta(), &MsgMeta::default());
}

#[tokio::test]
async fn test_update_msg() {
  let pool = init_db().await.expect("Failed to init database");
  let channel = ChannelBuilder::default()
    .name("test")
    .desc("test channel")
    .build()
    .unwrap();

  let mut msg = Msg::new(MsgRole::User, MsgCont::text_init(), MsgMeta::default());

  add_msg(&pool, channel.id(), &msg)
    .await
    .expect("Failed to add msg");

  msg.add_cont(MsgCont::text_init());

  update_msg(&pool, &msg).await.expect("Failed to update msg");

  let query_msg = query_msg_by_id(&pool, msg.id())
    .await
    .expect("Failed to query msg");

  assert_eq!(msg.id(), query_msg.id());
  assert_eq!(msg.cur_idx(), 1);
  assert_eq!(msg.cont_count(), 2);
  assert_eq!(msg.role(), &MsgRole::User);
  assert_eq!(msg.meta(), &MsgMeta::default());
}

#[tokio::test]
async fn test_query_msgs_by_channel_id() {
  let pool = init_db().await.expect("Failed to init database");
  let channel = ChannelBuilder::default()
    .name("test")
    .desc("test channel")
    .build()
    .unwrap();

  let msg1 = Msg::new(MsgRole::User, MsgCont::text_init(), MsgMeta::default());

  add_msg(&pool, channel.id(), &msg1)
    .await
    .expect("Failed to add msg1");

  let msg2 = Msg::new(MsgRole::User, MsgCont::text_init(), MsgMeta::default());

  add_msg(&pool, channel.id(), &msg2)
    .await
    .expect("Failed to add msg2");

  let query_msgs = query_msgs_by_channel_id(&pool, channel.id())
    .await
    .expect("Failed to query msg");

  assert_eq!(query_msgs.len(), 2);
  assert_eq!(query_msgs[0].id(), msg1.id());
  assert_eq!(query_msgs[1].id(), msg2.id());
}
