use uuid::Uuid;

use crate::{db::pool::DbPool, error::PolestarError, model::Attachment};

pub async fn add_attachment(pool: &DbPool, attachment: &Attachment) -> Result<Uuid, PolestarError> {
  let res = sqlx::query(
    r#"
    INSERT INTO attachment (name, mime, data)
    VALUES (?1, ?2, ?3)
    "#,
  )
  .bind(attachment.name())
  .bind(attachment.mime())
  .bind(attachment.data())
  .execute(pool)
  .await?;

  log::info!("add attachment result: {:?}", res);

  Ok(*attachment.name())
}

pub async fn query_attachment_by_name(
  pool: &DbPool,
  name: &Uuid,
) -> Result<Attachment, PolestarError> {
  let attachment = sqlx::query_as::<_, Attachment>(
    r#"
    SELECT name, mime, data
    FROM attachment
    WHERE name = ?1
    "#,
  )
  .bind(name)
  .fetch_one(pool)
  .await?;

  log::info!("query attachment result: {:?}", attachment);

  Ok(attachment)
}
