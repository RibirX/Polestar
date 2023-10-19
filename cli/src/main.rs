use std::sync::Mutex;

use once_cell::sync::Lazy;
use polestar_chat_core::{
  chat_data::DbBlockingClient,
  model::{
    app_data::{AppBuilder, AppData},
    channel::ChannelBuilder,
  },
};
use reedline_repl_rs::Result as ReplResult;

mod cli;

pub static APP_DATA: Lazy<Mutex<AppData<DbBlockingClient>>> = Lazy::new(|| {
  Mutex::new(
    AppBuilder::default()
      .build()
      .expect("build app store failed"),
  )
});

fn main() -> ReplResult<()> {
  let _ = APP_DATA
    .try_lock()
    .expect("get app data lock failed")
    .chat_data_mut()
    .add_channel(
      ChannelBuilder::default()
        .name("default")
        .desc("default channel")
        .build()
        .unwrap(),
    );

  tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .build()
    .unwrap()
    .block_on(async { cli::cli().await })
}
