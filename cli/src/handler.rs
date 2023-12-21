use inquire::Select;
use polestar_core::{
  model::{AppData, ChannelCfg, Msg, MsgAction, MsgBody, MsgCont, MsgMeta, MsgRole},
  service::open_ai::{deal_open_ai_stream, open_ai_stream},
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
      app_data.new_channel(name, desc, ChannelCfg::default());
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
        .find(|c| Some(c.id()) == app_data.info().cur_channel_id())
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
      let content = args.get_one::<String>("questions");
      {
        let cur_channel = app_data
          .cur_channel_mut()
          .expect("current channel not found");
        let mut msg_cont = MsgCont::init_text();
        msg_cont.action(MsgAction::Receiving(MsgBody::Text(
          content.map(|s| s.to_owned()),
        )));
        cur_channel.add_msg(Msg::new(MsgRole::User, vec![msg_cont], MsgMeta::default()));
      }

      let mut ret_msg = String::new();
      let bot = app_data.info().def_bot();
      let headers = bot.headers().try_into().unwrap_or_default();
      let url = bot.url().to_owned();
      let runtime = tokio::runtime::Runtime::new().unwrap();
      runtime.block_on(async {
        if let Ok(mut stream) =
          open_ai_stream(url, content.expect("content is required").clone(), headers).await
        {
          let res = deal_open_ai_stream(&mut stream, |s| print!("{}", s)).await;
          match res {
            Ok(s) => {
              ret_msg = s;
              println!();
            }
            Err(e) => println!("\nerror: {:?}", e),
          };
        }
      });

      {
        let cur_channel = app_data
          .cur_channel_mut()
          .expect("current channel not found");
        let mut msg_cont = MsgCont::init_text();
        msg_cont.action(MsgAction::Receiving(MsgBody::Text(Some(ret_msg.clone()))));
        cur_channel.add_msg(Msg::new(
          MsgRole::Bot(Uuid::nil()),
          vec![msg_cont],
          MsgMeta::default(),
        ));
      }

      Ok(None)
    }
    _ => Ok(None),
  }
}
