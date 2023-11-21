use std::{collections::HashMap, fs, io::Read};

use serde::Deserialize;

use super::{launch::write_default_bot_config, project_bot_config_path};
use crate::{model::Bot, service::service_provider::ServiceModel};

#[derive(Deserialize, Debug)]
pub struct BotCfg {
  pub bots: Vec<Bot>,
  pub tokens: Option<HashMap<String, String>>,
}

impl BotCfg {
  pub fn has_official_server(&self) -> bool {
    self.bots.iter().any(|b| b.sp() == &ServiceModel::Polestar)
  }
}

pub fn load_bot_cfg_file() -> BotCfg {
  let path = project_bot_config_path();

  fs::File::open(&path)
    .and_then(|mut file| {
      let mut content = String::new();
      file.read_to_string(&mut content).and_then(|_| {
        let bot_cfg = serde_json::from_str::<BotCfg>(&content)?;
        Ok(bot_cfg)
      })
    })
    .map_or_else(
      |_| {
        write_default_bot_config();
        let bot_cfg = serde_json::from_str::<BotCfg>(include_str!(concat!(
          env!("CARGO_MANIFEST_DIR"),
          "/..",
          "/config/bot.json"
        )))
        .expect("can't parse default bot.json");
        bot_cfg
      },
      |bots| bots,
    )
}
