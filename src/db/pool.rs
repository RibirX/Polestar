use crate::error::PolestarError;
use crate::utils::fs::user_data_path;
use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePool, Sqlite};

pub type DbPool = SqlitePool;

pub fn db_path() -> String {
  // TODO: user id
  let user_data_path = user_data_path(0);
  format!(
    "sqlite://{}/data.db?mode=rwc",
    user_data_path.to_str().unwrap()
  )
}

pub async fn init_setup_db(db_path: &str) -> Result<(), PolestarError> {
  Sqlite::create_database(db_path).await?;
  log::info!("Init user database success!");
  let pool = db_pool(db_path).await?;
  // Migrate the database
  let res = sqlx::migrate!("src/db/migrations").run(&pool).await;
  log::info!("Migrate database result: {:?}", res);
  Ok(())
}

pub async fn db_pool(db_path: &str) -> Result<DbPool, PolestarError> {
  let pool = SqlitePool::connect(db_path).await?;
  log::info!("Get user database connect success!");
  Ok(pool)
}
