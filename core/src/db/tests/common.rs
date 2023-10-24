use crate::{db::pool::DbPool, error::PolestarResult};
use sqlx::{migrate::MigrateDatabase, Sqlite};

pub async fn init_db() -> PolestarResult<DbPool> {
  Sqlite::create_database("sqlite::memory:").await?;
  let pool = sqlx::pool::PoolOptions::<Sqlite>::new()
    .max_lifetime(None)
    .idle_timeout(None)
    .connect("sqlite::memory:")
    .await?;
  let _ = sqlx::migrate!("src/db/migrations").run(&pool).await;
  Ok(pool)
}
