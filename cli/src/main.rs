use handler::{channel_handler, msg_handler};
use polestar_core::model::AppData;
use reedline_repl_rs::clap::{Arg, Command};
use reedline_repl_rs::{Repl, Result as ReplResult};

mod handler;

static VERSION: &str = env!("CARGO_PKG_VERSION");
static APP_NAME: &str = env!("CARGO_PKG_NAME");
static APP_DESC: &str = env!("CARGO_PKG_DESCRIPTION");

fn main() -> ReplResult<()> {
  let mut app_data = AppData::new(vec![], uuid::Uuid::nil());
  app_data.new_channel("quick launcher".to_owned(), None);
  let mut repl = Repl::new(app_data)
    .with_name(APP_NAME)
    .with_version(&format!("v{}", VERSION))
    .with_description(APP_DESC)
    .with_banner("Welcome to Polestar!")
    .with_command(
      Command::new("channel")
        .subcommands([
          Command::new("all").about("Show all channels"),
          Command::new("current").about("Show current channel"),
          Command::new("remove").about("Remove channel"),
          Command::new("add")
            .arg(Arg::new("name").required(true))
            .arg(Arg::new("desc"))
            .about("Add channel"),
          Command::new("switch").about("Switch channel"),
        ])
        .arg_required_else_help(true),
      channel_handler,
    )
    .with_command(
      Command::new("msg").subcommands([Command::new("send")
        .arg(Arg::new("questions").required(true))
        .about("Send message")]),
      msg_handler,
    );
  repl.run()
}
