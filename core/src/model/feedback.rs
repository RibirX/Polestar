use serde::{Deserialize, Serialize};

use super::{Msg, MsgCont, MsgMeta, MsgRole};

impl From<FeedbackMessageForServer> for Msg {
  fn from(msg: FeedbackMessageForServer) -> Self {
    let role = match msg.id {
      FeedbackUserIdForServer::UserId(_) => MsgRole::User,
      FeedbackUserIdForServer::System(id) => MsgRole::System(id),
    };
    Msg::new(
      role,
      vec![MsgCont::new_text(&msg.message)],
      MsgMeta::default(),
    )
  }
}

type UserId = u64;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FeedbackUserIdForServer {
  UserId(UserId),
  System(UserId),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserFeedbackMessageForServer {
  pub message: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FeedbackMessageForServer {
  pub id: FeedbackUserIdForServer,
  pub message: String,
  pub create_time: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FeedbackTimestamp {
  pub create_time: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FeedbackMessageListForServer {
  pub data: Vec<FeedbackMessageForServer>,
  pub next: Option<u64>,
}
