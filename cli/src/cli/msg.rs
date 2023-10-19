use polestar_chat_core::service::open_ai::stream_string;
use reedline_repl_rs::{clap::ArgMatches, Result as ReplResult};

pub async fn msg_handler<T>(args: ArgMatches, _context: &mut T) -> ReplResult<Option<String>> {
  match args.subcommand() {
    Some(("send", args)) => {
      let content = args.get_one::<String>("questions").unwrap();
      let ret_msg = stream_string(content).await;

      Ok(Some(ret_msg))
    }
    _ => Ok(None),
  }
}
