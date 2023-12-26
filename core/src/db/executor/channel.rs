use sqlx::Row;
use uuid::Uuid;

use crate::{
  db::pool::DbPool,
  error::PolestarError,
  model::{Channel, ChannelCfg, ChannelId},
};

pub async fn add_channel(
  pool: &DbPool,
  id: &ChannelId,
  name: &str,
  desc: Option<&str>,
  cfg: &ChannelCfg,
) -> Result<(), PolestarError> {
  let cfg = serde_json::to_string(cfg)?;
  let res = sqlx::query(
    r#"
    INSERT INTO channel (id, name, desc, cfg)
    VALUES (?1, ?2, ?3, ?4)
    "#,
  )
  .bind(id)
  .bind(name)
  .bind(desc)
  .bind(cfg)
  .execute(pool)
  .await?;

  log::info!("add channel result: {:?}", res);

  Ok(())
}

pub async fn remove_channel(pool: &DbPool, id: &Uuid) -> Result<(), PolestarError> {
  let res = sqlx::query(
    r#"
    DELETE FROM channel
    WHERE id = ?1
    "#,
  )
  .bind(id)
  .execute(pool)
  .await?;

  log::info!("remove channel result: {:?}", res);

  let res = sqlx::query(
    r#"
    DELETE FROM msg
    WHERE channel_id = ?1
    "#,
  )
  .bind(id)
  .execute(pool)
  .await?;

  log::info!("remove channel msgs result: {:?}", res);

  Ok(())
}

pub async fn query_channel_by_id(pool: &DbPool, id: &Uuid) -> Result<Channel, PolestarError> {
  let row = sqlx::query(
    r#"
    SELECT id, name, desc, cfg
    FROM channel
    WHERE id = ?1
    "#,
  )
  .bind(id)
  .fetch_one(pool)
  .await?;

  let channel = Channel::new(
    row.get(0),
    row.get(1),
    row.get(2),
    serde_json::from_str(row.get(3))?,
    None,
    None,
  );

  log::info!("query channel result: {:?}", channel);

  Ok(channel)
}

pub async fn update_channel(
  pool: &DbPool,
  id: &ChannelId,
  name: &str,
  desc: Option<&str>,
  cfg: &ChannelCfg,
) -> Result<(), PolestarError> {
  let cfg = serde_json::to_string(cfg)?;
  let res = sqlx::query(
    r#"
    UPDATE channel
    SET name = ?1, desc = ?2, cfg = ?3
    WHERE id = ?4
    "#,
  )
  .bind(name)
  .bind(desc)
  .bind(cfg)
  .bind(id)
  .execute(pool)
  .await?;

  log::info!("update channel result: {:?}", res);

  Ok(())
}

pub async fn query_channels(pool: &DbPool) -> Result<Vec<Channel>, PolestarError> {
  let rows = sqlx::query(
    r#"
    SELECT id, name, desc, cfg
    FROM channel
    "#,
  )
  .fetch_all(pool)
  .await?;

  let mut channels = vec![];

  for row in rows {
    let channel = Channel::new(
      row.get(0),
      row.get(1),
      row.get(2),
      serde_json::from_str(row.get(3))?,
      None,
      None,
    );

    channels.push(channel);
  }

  log::info!("query channels result: {:?}", channels);

  Ok(channels)
}
