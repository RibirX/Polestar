use std::str::FromStr;

use inquire::Select;
use reedline_repl_rs::{clap::ArgMatches, Result as ReplResult};
use uuid::Uuid;

use polestar_chat_core::model::channel::ChannelBuilder;

use crate::APP_DATA;

pub async fn channel_handler<T>(args: ArgMatches, _context: &mut T) -> ReplResult<Option<String>> {
  match args.subcommand() {
    Some(("all", _args)) => {
      let app_data = APP_DATA.try_lock().expect("get app data lock failed");
      app_data
        .chat_data_ref()
        .channels()
        .iter()
        .for_each(|channel| {
          println!("{}: {}", channel.id(), channel.name());
        });
      Ok(Some("".to_owned()))
    }
    Some(("add", args)) => {
      let name = args.get_one::<String>("name").unwrap();
      let desc = args.get_one::<String>("desc");
      let mut channel_builder = ChannelBuilder::default();
      channel_builder.name(name);
      if let Some(desc) = desc {
        channel_builder.desc(desc);
      }

      let _ = APP_DATA
        .lock()
        .unwrap()
        .chat_data_mut()
        .add_channel(channel_builder.build().unwrap());
      Ok(Some("add".to_owned()))
    }
    Some(("remove", _)) => {
      let result = tokio::task::spawn_blocking(|| {
        let selects: Vec<String> = APP_DATA
          .lock()
          .unwrap()
          .chat_data_ref()
          .channels()
          .iter()
          .map(|channel| format!("{}: {}", channel.id(), channel.name()))
          .collect();
        Select::new("Operate ›", selects)
          .with_help_message("↑↓ to move, enter to select, type to filter, Esc to quit")
          .prompt_skippable()
      })
      .await
      .unwrap()
      .unwrap();
      if let Some(select) = result {
        let channel_id = select.split(':').next().unwrap().trim();

        let _ = APP_DATA
          .lock()
          .unwrap()
          .chat_data_mut()
          .remove_channel(&Uuid::from_str(channel_id).expect("invalid channel id"));

        Ok(Some(format!("remove {}", select)))
      } else {
        Ok(None)
      }
    }
    Some(("current", _)) => {
      let app_data = APP_DATA.lock().unwrap();
      let cur_channel_id = app_data.chat_data_ref().cur_channel_id();
      let channel = app_data
        .chat_data_ref()
        .channels()
        .iter()
        .find(|channel| channel.id() == cur_channel_id)
        .unwrap();
      println!("{}: {}", channel.id(), channel.name());
      Ok(Some("".to_owned()))
    }
    Some(("switch", _)) => {
      let result = tokio::task::spawn_blocking(|| {
        let selects: Vec<String> = APP_DATA
          .lock()
          .unwrap()
          .chat_data_ref()
          .channels()
          .iter()
          .map(|channel| format!("{}: {}", channel.id(), channel.name()))
          .collect();
        Select::new("Operate ›", selects)
          .with_help_message("↑↓ to move, enter to select, type to filter, Esc to quit")
          .prompt_skippable()
      })
      .await
      .unwrap()
      .unwrap();
      if let Some(select) = result {
        let channel_id = select.split(':').next().unwrap().trim();

        let _ = APP_DATA
          .lock()
          .unwrap()
          .chat_data_mut()
          .switch_channel(&Uuid::from_str(channel_id).expect("invalid channel id"));
      }
      Ok(Some("switch".to_owned()))
    }
    _ => Ok(None),
  }
}
