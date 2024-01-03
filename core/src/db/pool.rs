use std::marker::PhantomPinned;

use crate::model::{Msg, ANONYMOUS_USER};
use crate::utils::user_data_path;
use crate::{error::PolestarResult, model::Channel};
use once_cell::sync::Lazy;
use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePool, Sqlite};
use tokio::{runtime::Handle, sync::mpsc::UnboundedSender};
use uuid::Uuid;

use super::executor::{ActionPersist, Persist};

pub type DbPool = SqlitePool;

pub fn db_path(uid: Option<u64>) -> String {
  let uid = uid
    .map(|uid| uid.to_string())
    .unwrap_or(ANONYMOUS_USER.to_owned());
  let user_data_path = user_data_path(&uid);
  format!(
    "sqlite://{}/data.db?mode=rwc",
    user_data_path.to_str().unwrap()
  )
}

pub fn runtime() -> Handle {
  static RT: Lazy<tokio::runtime::Runtime> = Lazy::new(|| {
    tokio::runtime::Builder::new_multi_thread()
      .worker_threads(2)
      .enable_all()
      .build()
      .unwrap()
  });
  RT.handle().clone()
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

pub struct PersistenceDB {
  inner: DbPool,
  sender: UnboundedSender<ActionPersist>,
  _marker: PhantomPinned,
}

impl PersistenceDB {
  pub fn connect(
    init_db: impl std::future::Future<Output = PolestarResult<DbPool>>,
  ) -> PolestarResult<Self> {
    let inner = runtime().block_on(init_db)?;
    let db = inner.clone();
    let (sender, mut receiver) = tokio::sync::mpsc::unbounded_channel::<ActionPersist>();
    runtime().spawn(async move {
      while let Some(msg) = receiver.recv().await {
        msg.write(&db).await.expect("Failed to write persist");
      }
    });
    Ok(Self {
      inner,
      sender,
      _marker: PhantomPinned,
    })
  }

  pub fn persist_async(&self, persist: ActionPersist) { let _ = self.sender.send(persist); }

  pub async fn query_channels(&self) -> PolestarResult<Vec<Channel>> {
    super::executor::channel::query_channels(&self.inner).await
  }

  pub async fn query_msgs_by_channel_id(
    &self,
    channel_id: &uuid::Uuid,
  ) -> PolestarResult<Vec<Msg>> {
    super::executor::msg::query_msgs_by_channel_id(&self.inner, channel_id).await
  }

  pub async fn query_attachment_by_name(
    &self,
    name: &Uuid,
  ) -> PolestarResult<crate::model::Attachment> {
    super::executor::attachment::query_attachment_by_name(&self.inner, name).await
  }
}
