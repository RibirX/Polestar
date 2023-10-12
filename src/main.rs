use std::sync::Mutex;

use model::{
  app_store::{AppBuilder, AppStore},
  channel::ChannelBuilder,
};
use once_cell::sync::Lazy;
use reedline_repl_rs::Result as ReplResult;

mod cli;
mod db;
mod error;
mod model;
mod service;
mod utils;

pub static APP_STORE: Lazy<Mutex<AppStore>> = Lazy::new(|| {
  Mutex::new(
    AppBuilder::default()
      .build()
      .expect("build app store failed"),
  )
});

#[tokio::main]
async fn main() -> ReplResult<()> {
  // default add channel
  APP_STORE.lock().unwrap().add_channel(
    ChannelBuilder::default()
      .name("default")
      .desc("default channel")
      .build()
      .unwrap(),
  );

  cli::cli().await
}
