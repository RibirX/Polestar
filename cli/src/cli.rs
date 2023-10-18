pub mod channel;
pub mod msg;

use channel::channel_handler;
use msg::msg_handler;
use reedline_repl_rs::clap::{Arg, Command};
use reedline_repl_rs::{Repl, Result as ReplResult};

static VERSION: &str = env!("CARGO_PKG_VERSION");
static APP_NAME: &str = env!("CARGO_PKG_NAME");
static APP_DESC: &str = env!("CARGO_PKG_DESCRIPTION");

pub async fn cli() -> ReplResult<()> {
  let mut repl = Repl::new(())
    .with_name(APP_NAME)
    .with_version(&format!("v{}", VERSION))
    .with_description(APP_DESC)
    .with_banner("Welcome to Polestar!")
    .with_command_async(
      Command::new("msg").subcommands([Command::new("send")
        .arg(Arg::new("questions").required(true))
        .about("Send message")]),
      |args, context| Box::pin(msg_handler(args, context)),
    )
    .with_command_async(
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
      |args, context| Box::pin(channel_handler(args, context)),
    );
  repl.run_async().await
}
