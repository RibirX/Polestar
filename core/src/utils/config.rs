use crate::model::Bot;

pub fn load_bot_cfg_file() -> Vec<Bot> {
  let preset_bot_cfg = serde_json::from_str::<serde_json::Value>(include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/..",
    "/config/bot.json"
  )))
  .expect("Failed to parse bot.json");
  let bots = serde_json::from_value::<Vec<Bot>>(preset_bot_cfg["bots"].clone())
    .expect("Failed to parse bots");
  bots
}
