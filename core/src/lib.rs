pub mod db;
pub mod error;
pub mod model;
pub mod service;
mod utils;

// TODO: pub need double check.
pub use utils::*;
pub static KEY: &[u8] = include_bytes!("../../key");
