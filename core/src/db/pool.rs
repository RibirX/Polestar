use std::marker::PhantomPinned;
use std::pin::Pin;
use std::time::Duration;

use crate::utils::user_data_path;
use crate::{error::PolestarResult, model::Channel};
use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePool, Sqlite};
use uuid::Uuid;

use super::executor::{ActionPersist, Persist};

pub type DbPool = SqlitePool;

pub fn db_path() -> String {
  // TODO: user id
  let user_data_path = user_data_path("anonymous");
  format!(
    "sqlite://{}/data.db?mode=rwc",
    user_data_path.to_str().unwrap()
  )
}

pub async fn init_db(db_path: &str) -> PolestarResult<DbPool> {
  Sqlite::create_database(db_path).await?;
  log::info!("Init user database success!");
  let pool = SqlitePool::connect(db_path).await?;
  // Migrate the database
  let res = sqlx::migrate!("src/db/migrations").run(&pool).await;
  log::info!("Migrate database result: {:?}", res);
  Ok(pool)
}

#[derive(PartialEq, Eq)]
pub enum PersistStatus {
  Doing,
  Done,
}

pub struct PersistenceDB {
  rt: tokio::runtime::Runtime,
  inner: DbPool,
  list: Vec<ActionPersist>,
  status: PersistStatus,
  timeout: Duration,
  _marker: PhantomPinned,
}

impl PersistenceDB {
  pub fn connect(
    init_db: impl std::future::Future<Output = PolestarResult<DbPool>>,
    timeout: Duration,
  ) -> PolestarResult<Self> {
    let rt = tokio::runtime::Builder::new_multi_thread()
      .worker_threads(2)
      .enable_all()
      .build()?;
    let inner = rt.block_on(init_db)?;
    Ok(Self {
      rt,
      inner,
      list: vec![],
      status: PersistStatus::Done,
      timeout,
      _marker: PhantomPinned,
    })
  }

  pub fn run_batch_timeout(&mut self, timeout: Duration) {
    self.status = PersistStatus::Doing;
    self.rt.block_on(async {
      tokio::time::sleep(timeout).await;
      while let Some(persist) = self.list.pop() {
        persist
          .write(&self.inner)
          .await
          .expect("Failed to write persist");
      }
      self.status = PersistStatus::Done;
    })
  }

  pub fn add_persist(self: Pin<&mut Self>, persist: ActionPersist) {
    let db = unsafe { self.get_unchecked_mut() };
    db.list.push(persist);
    if db.status == PersistStatus::Done {
      db.run_batch_timeout(db.timeout);
    }
  }

  pub fn query_channels(&self) -> PolestarResult<Vec<Channel>> {
    self
      .rt
      .block_on(super::executor::channel::query_channels(&self.inner))
  }

  pub fn query_msgs_by_channel_id(
    &self,
    channel_id: &uuid::Uuid,
  ) -> PolestarResult<Vec<crate::model::Msg>> {
    self
      .rt
      .block_on(super::executor::msg::query_msgs_by_channel_id(
        &self.inner,
        channel_id,
      ))
  }

  pub fn query_attachment_by_name(&self, name: &Uuid) -> PolestarResult<crate::model::Attachment> {
    self
      .rt
      .block_on(super::executor::attachment::query_attachment_by_name(
        &self.inner,
        name,
      ))
  }
}
