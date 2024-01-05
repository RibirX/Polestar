use uuid::Uuid;

use crate::{db::pool::DbPool, error::PolestarError, model::Msg};

pub async fn add_msg(pool: &DbPool, channel_id: &Uuid, msg: &Msg) -> Result<(), PolestarError> {
  let role = serde_json::to_string(msg.role())?;
  let cont_list = serde_json::to_string(msg.cont_list())?;
  let meta = serde_json::to_string(msg.meta())?;
  let res = sqlx::query(
    r#"
    INSERT INTO msg (id, channel_id, role, cur_idx, cont_list, meta, created_at)
    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
    "#,
  )
  .bind(msg.id())
  .bind(channel_id)
  .bind(role)
  .bind(msg.cur_idx() as i32)
  .bind(cont_list)
  .bind(meta)
  .bind(msg.create_at())
  .execute(pool)
  .await?;

  log::info!("add msg result: {:?}", res);

  Ok(())
}

pub async fn query_msg_by_id(pool: &DbPool, id: &Uuid) -> Result<Msg, PolestarError> {
  let msg = sqlx::query_as::<_, Msg>(
    r#"
    SELECT id, role, cur_idx, cont_list, meta, created_at
    FROM msg
    WHERE id = ?1
    "#,
  )
  .bind(id)
  .fetch_one(pool)
  .await?;

  log::info!("query msg result: {:?}", msg);

  Ok(msg)
}

// TODO: update method need split? or use one method?
pub async fn update_msg(pool: &DbPool, msg: &Msg) -> Result<(), PolestarError> {
  let cont_list = serde_json::to_string(msg.cont_list())?;
  let meta = serde_json::to_string(msg.meta())?;
  let res = sqlx::query(
    r#"
    UPDATE msg
    SET cur_idx = ?1, cont_list = ?2, meta = ?3
    WHERE id = ?4
    "#,
  )
  .bind(msg.cur_idx() as i32)
  .bind(cont_list)
  .bind(meta)
  .bind(msg.id())
  .execute(pool)
  .await?;

  log::info!("update msg result: {:?}", res);

  Ok(())
}

pub async fn remove_msg(pool: &DbPool, id: &Uuid) -> Result<(), PolestarError> {
  let res = sqlx::query(
    r#"
    DELETE FROM msg
    WHERE id = ?1
    "#,
  )
  .bind(id)
  .execute(pool)
  .await?;

  log::info!("remove msg result: {:?}", res);

  Ok(())
}

pub async fn query_msgs_by_channel_id(
  pool: &DbPool,
  channel_id: &Uuid,
) -> Result<Vec<Msg>, PolestarError> {
  let msgs = sqlx::query_as::<_, Msg>(
    r#"
    SELECT id, role, cur_idx, cont_list, meta, created_at
    FROM msg
    WHERE channel_id = ?1
    "#,
  )
  .bind(channel_id)
  .fetch_all(pool)
  .await?;

  log::info!("query msgs result: {:?}", msgs);

  Ok(msgs)
}

pub async fn query_latest_text_msgs_by_channel_id(
  _pool: &DbPool,
  _channel_id: &Uuid,
  _limit: u32,
) -> Result<Vec<Msg>, PolestarError> {
  todo!()
}
