use std::path::PathBuf;

use home::home_dir;

use crate::error::PolestarResult;

use super::LocalState;

const POLESTAR_FOLDER: &str = ".polestar_v1";
const USERS_FOLDER: &str = "users";
const CONFIG_FOLDER: &str = "config";
const BOT_CONFIG_FILE: &str = "bot.json";
const CURRENT_USER: &str = "current_user";
const NONCE_FILE: &str = "nonce";
const TOKEN_FILE: &str = "token";
const LOCAL_STATE: &str = "local_state";

pub fn project_home_path() -> PathBuf {
  home_dir()
    .map(|mut path| {
      path.push(POLESTAR_FOLDER);
      path
    })
    .expect("Failed to get home directory path")
}

pub fn user_data_path(uid: &str) -> PathBuf {
  let mut path = project_user_path();
  path.push(uid);
  path
}

fn project_config_path() -> PathBuf {
  let mut path = project_home_path();
  path.push(CONFIG_FOLDER);
  path
}

pub(super) fn project_bot_config_path() -> PathBuf {
  let mut path = project_config_path();
  path.push(BOT_CONFIG_FILE);
  path
}

fn project_user_path() -> PathBuf {
  let mut path = project_home_path();
  path.push(USERS_FOLDER);
  path
}

pub fn read_current_user() -> PolestarResult<String> {
  let mut path = project_home_path();
  path.push(CURRENT_USER);
  let content = std::fs::read_to_string(&path)?;
  Ok(content)
}

pub fn write_current_user(uid: &str) -> PolestarResult<()> {
  let mut path = project_home_path();
  path.push(CURRENT_USER);
  std::fs::write(&path, uid)?;
  Ok(())
}

pub fn read_local_state() -> PolestarResult<LocalState> {
  let mut path = project_home_path();
  path.push(LOCAL_STATE);
  let content = std::fs::read_to_string(&path)?;
  let local_state = serde_json::from_str::<LocalState>(&content)?;
  Ok(local_state)
}

pub fn write_local_state(local_state: &LocalState) -> PolestarResult<()> {
  let mut path = project_home_path();
  path.push(LOCAL_STATE);
  let content = serde_json::to_string_pretty(local_state)?;
  std::fs::write(&path, content)?;
  Ok(())
}

pub fn create_if_not_exist_dir(path: PathBuf) {
  if let Err(err) = std::fs::metadata(&path) {
    if err.kind() == std::io::ErrorKind::NotFound {
      std::fs::create_dir(&path).expect("Failed to create user data directory");
    } else {
      // TODO: need tip user.
      panic!("Init User Folder {:?} Error {}", path, err);
    }
  }
}

pub mod encrypt {
  use std::{fs::File, io::Write};

  use crate::error::PolestarResult;

  use super::*;

  pub fn read_token() -> PolestarResult<Vec<u8>> {
    let mut token_path = project_home_path();
    token_path.push(TOKEN_FILE);
    let token = std::fs::read(&token_path)?;
    Ok(token)
  }

  pub fn write_token(token: &[u8]) -> PolestarResult<()> {
    let mut token_path = project_home_path();
    token_path.push(TOKEN_FILE);
    let mut buffer = File::create(token_path)?;
    buffer.write_all(token)?;
    Ok(())
  }

  pub fn del_token() -> PolestarResult<()> {
    let mut token_path = project_home_path();
    token_path.push(TOKEN_FILE);
    std::fs::remove_file(&token_path)?;
    Ok(())
  }

  pub fn read_nonce() -> PolestarResult<Vec<u8>> {
    let mut nonce_path = project_home_path();
    nonce_path.push(NONCE_FILE);
    let nonce = std::fs::read(&nonce_path)?;
    Ok(nonce)
  }

  pub fn write_nonce(nonce: &[u8]) -> PolestarResult<()> {
    let mut nonce_path = project_home_path();
    nonce_path.push(NONCE_FILE);
    let mut buffer = File::create(nonce_path)?;
    buffer.write_all(nonce)?;
    Ok(())
  }

  pub fn del_nonce() -> PolestarResult<()> {
    let mut nonce_path = project_home_path();
    nonce_path.push(NONCE_FILE);
    std::fs::remove_file(&nonce_path)?;
    Ok(())
  }
}

pub mod launch {
  use std::{fs, io::Write};

  use super::{
    create_if_not_exist_dir, project_bot_config_path, project_config_path, project_home_path,
    project_user_path,
  };

  pub fn setup_project() {
    create_if_not_exist_dir(project_home_path());
    create_if_not_exist_dir(project_user_path());
    create_if_not_exist_dir(project_config_path());
    write_default_bot_config();
  }

  pub fn write_default_bot_config() {
    let path = project_bot_config_path();

    if fs::metadata(&path).is_err() {
      let content = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/..",
        "/config/bot.json"
      ));
      let mut file = fs::File::create(path).expect("can't create bot.json");
      file
        .write_all(content.as_bytes())
        .expect("can't write bot.json");
    }
  }
}
