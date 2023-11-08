use inquire::Select;
use polestar_core::{
  model::{AppData, Msg, MsgAction, MsgBody, MsgCont, MsgMeta, MsgRole},
  service::open_ai::stream_string,
};
use reedline_repl_rs::{clap::ArgMatches, Result as ReplResult};
use uuid::Uuid;

pub fn channel_handler(args: ArgMatches, app_data: &mut AppData) -> ReplResult<Option<String>> {
  match args.subcommand() {
    Some(("all", _args)) => {
      let channels = app_data.channels();
      for channel in channels {
        println!("{}: {}", channel.id(), channel.name());
      }
      Ok(None)
    }
    Some(("add", args)) => {
      let name = args
        .get_one::<String>("name")
        .expect("name is required")
        .to_owned();
      let desc = args.get_one::<String>("desc").map(|s| s.to_owned());
      app_data.new_channel(name, desc);
      Ok(None)
    }
    Some(("remove", _args)) => {
      let channels = app_data
        .channels()
        .iter()
        .map(|channel| format!("{}: {}", channel.id(), channel.name()))
        .collect::<Vec<String>>();
      let rst = Select::new("Operate ›", channels)
        .with_help_message("↑↓ to move, enter to select, type to filter, Esc to quit")
        .prompt_skippable()
        .expect("failed to prompt");

      if let Some(select) = rst {
        let channel_id = select.split(':').next().expect("invalid channel id");
        let channel_id = Uuid::parse_str(channel_id).expect("invalid channel id");
        app_data.remove_channel(&channel_id);
      }
      Ok(None)
    }
    Some(("current", _args)) => {
      let channel = app_data
        .channels()
        .iter()
        .find(|c| c.id() == app_data.cur_channel_id())
        .expect("current channel not found");
      println!("{}: {}", channel.id(), channel.name());
      Ok(None)
    }
    Some(("switch", _args)) => {
      let channels = app_data
        .channels()
        .iter()
        .map(|channel| format!("{}: {}", channel.id(), channel.name()))
        .collect::<Vec<String>>();
      let rst = Select::new("Operate ›", channels)
        .with_help_message("↑↓ to move, enter to select, type to filter, Esc to quit")
        .prompt_skippable()
        .expect("failed to prompt");

      if let Some(select) = rst {
        let channel_id = select.split(':').next().expect("invalid channel id");
        let channel_id = Uuid::parse_str(channel_id).expect("invalid channel id");
        app_data.switch_channel(&channel_id);
      }
      Ok(None)
    }
    _ => Ok(None),
  }
}

pub fn msg_handler(args: ArgMatches, app_data: &mut AppData) -> ReplResult<Option<String>> {
  match args.subcommand() {
    Some(("send", args)) => {
      let cur_channel = app_data
        .cur_channel_mut()
        .expect("current channel not found");
      let content = args.get_one::<String>("questions");
      let msg_cont = MsgCont::text_init();
      let msg_cont = msg_cont.action(MsgAction::Receiving(MsgBody::Text(
        content.map(|s| s.to_owned()),
      )));
      cur_channel.add_msg(Msg::new(MsgRole::User, vec![msg_cont], MsgMeta::default()));
      let runtime = tokio::runtime::Runtime::new().unwrap();
      let ret_msg = runtime.block_on(stream_string(content.expect("content is required")));
      let msg_cont = MsgCont::text_init();
      let msg_cont = msg_cont.action(MsgAction::Receiving(MsgBody::Text(Some(ret_msg.clone()))));
      cur_channel.add_msg(Msg::new(
        MsgRole::Bot(Uuid::nil()),
        vec![msg_cont],
        MsgMeta::default(),
      ));

      Ok(Some(ret_msg))
    }
    _ => Ok(None),
  }
}
