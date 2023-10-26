use std::{fs, io::ErrorKind, path::PathBuf};

use home::home_dir;

const POLESTAR_FOLDER: &str = ".polestar_v1";
const USERS_FOLDER: &str = "users";

pub fn project_home_path() -> PathBuf {
  home_dir()
    .map(|mut path| {
      path.push(POLESTAR_FOLDER);
      path
    })
    .expect("Failed to get home directory path")
}

pub fn user_data_path(user_id: &str) -> PathBuf {
  let mut path = project_user_path();
  path.push(user_id);
  path
}

pub fn setup_user_dir(user_id: &str) {
  let user_path = user_data_path(user_id);

  if let Err(err) = fs::metadata(&user_path) {
    if err.kind() == ErrorKind::NotFound {
      fs::create_dir(&user_path).expect("Failed to create user data directory");
    } else {
      panic!("Init User Folder Error {}", err);
    }
  }
}

pub fn setup_project() {
  init_project_dir();
  init_project_user_dir();
}

fn project_user_path() -> PathBuf {
  let mut path = project_home_path();
  path.push(USERS_FOLDER);
  path
}

// Project directory initialization
fn init_project_dir() {
  let home_path = project_home_path();

  if let Err(err) = fs::metadata(&home_path) {
    if err.kind() == ErrorKind::NotFound {
      fs::create_dir(&home_path).expect("Failed to create project home directory");
    } else {
      panic!("Init PoleStar Folder Error {}", err);
    }
  }
}

// User directory initialization
fn init_project_user_dir() {
  let user_path = project_user_path();

  if let Err(err) = fs::metadata(&user_path) {
    if err.kind() == ErrorKind::NotFound {
      fs::create_dir(&user_path).expect("Failed to create user data directory");
    } else {
      panic!("Init User Folder Error {}", err);
    }
  }
}
