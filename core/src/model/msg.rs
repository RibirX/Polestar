use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// `Message` is the message struct.
/// It contains the message role, message active_index and message content list.
#[derive(Debug, Serialize, Deserialize, Default, sqlx::FromRow, Clone)]
pub struct Msg {
  id: Uuid,
  // role is the sender of the message.
  #[sqlx(json)]
  role: MsgRole,
  // `cur_idx` is the index of the current message content show which one.
  cur_idx: i32,
  // `cont` is the message content list.
  #[sqlx(json)]
  cont_list: Vec<MsgCont>,
  // `meta` is the message meta data.
  #[sqlx(json)]
  meta: MsgMeta,
}

impl Msg {
  #[inline]
  pub fn id(&self) -> &Uuid { &self.id }

  #[inline]
  pub fn cur_idx(&self) -> usize { self.cur_idx as _ }

  #[inline]
  pub fn role(&self) -> &MsgRole { &self.role }

  #[inline]
  pub fn cur_cont_ref(&self) -> &MsgCont { &self.cont_list[self.cur_idx as usize] }

  #[inline]
  pub fn cur_cont_mut(&mut self) -> &mut MsgCont { &mut self.cont_list[self.cur_idx as usize] }

  #[inline]
  pub fn meta(&self) -> &MsgMeta { &self.meta }

  #[inline]
  pub fn cont_list(&self) -> &Vec<MsgCont> { &self.cont_list }
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq, Clone)]
pub struct MsgMeta {
  quote_id: Option<Uuid>,
  source_id: Option<Uuid>,
}

impl MsgMeta {
  #[inline]
  pub fn quote_id(&self) -> Option<&Uuid> { self.quote_id.as_ref() }

  #[inline]
  pub fn source_id(&self) -> Option<&Uuid> { self.source_id.as_ref() }
}

impl Msg {
  pub fn new(role: MsgRole, cont_list: Vec<MsgCont>, meta: MsgMeta) -> Self {
    Self {
      id: Uuid::new_v4(),
      role,
      cur_idx: 0,
      cont_list,
      meta,
    }
  }

  #[inline]
  pub fn add_cont(&mut self, cont: MsgCont) {
    let last_idx = self.cont_list.len();
    self.cont_list.push(cont);
    self.cur_idx = last_idx as _;
  }

  #[inline]
  pub fn update_cur_cont(&mut self, cont: MsgCont) { self.cont_list[self.cur_idx as usize] = cont; }

  pub fn update_cont(&mut self, idx: usize, cont: MsgCont) {
    let len = self.cont_list.len();
    if idx >= len {
      log::warn!(
        "[polestar] update message content error: index out of range: message's len is {}, but the index is {}",
        len,
        idx
      );
      return;
    }
    self.cont_list[idx] = cont;
  }

  pub fn switch_cont(&mut self, idx: usize) {
    let len = self.cont_list.len();
    if idx >= len {
      log::warn!(
        "[polestar] switch message content error: index out of range: message's len is {}, but the index is {}",
        len,
        idx
      );
      return;
    }
    self.cur_idx = idx as _;
  }

  #[inline]
  pub fn cont_count(&self) -> usize { self.cont_list.len() }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Image {
  width: u32,
  height: u32,
  path: ImagePath,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
enum ImagePath {
  #[serde(rename = "url")]
  Url(String),
  #[serde(rename = "file")]
  File(Uuid),
  #[serde(rename = "static")]
  Static(String),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum MsgBody {
  #[serde(rename = "text")]
  Text(Option<String>),
  #[serde(rename = "image")]
  Image(Option<Image>),
}

impl MsgBody {
  pub fn is_some(&self) -> bool {
    match self {
      Self::Text(s) => s.is_some(),
      Self::Image(i) => i.is_some(),
    }
  }

  pub fn is_none(&self) -> bool {
    match self {
      Self::Text(s) => s.is_none(),
      Self::Image(i) => i.is_none(),
    }
  }

  pub fn set_none(&mut self) {
    match self {
      Self::Text(s) => *s = None,
      Self::Image(i) => *i = None,
    }
  }

  pub fn receive_text(&mut self, text: Option<String>) {
    match self {
      Self::Text(s) => {
        if let Some(text) = text {
          if let Some(str) = s.as_mut() {
            *s = Some(format!("{}{}", str, text));
          } else {
            *s = Some(text);
          }
        }
      }
      Self::Image(i) => *i = None,
    }
  }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MsgCont {
  body: MsgBody,
  status: MsgStatus,
}

pub enum MsgAction {
  Pending,
  Fulfilled,
  Receiving(MsgBody),
  Rejected,
}

impl MsgCont {
  const PENDING_WARN: &'static str =
    "[polestar] message content action error: pending action must be rejected status";
  const FULFILLED_WARN: &'static str =
    "[polestar] message content action error: fulfilled action must be receiving status";
  const REJECTED_WARN: &'static str =
    "[polestar] message content action error: rejected action must be pending status";
  const RECEIVING_WARN: &'static str =
    "[polestar] message content action error: receiving action must be pending status";
  pub fn text_init() -> Self {
    Self {
      body: MsgBody::Text(None),
      status: MsgStatus::Pending,
    }
  }

  pub fn image_init() -> Self {
    Self {
      body: MsgBody::Image(None),
      status: MsgStatus::Pending,
    }
  }

  pub fn action(&mut self, action: MsgAction) {
    match action {
      MsgAction::Pending => {
        if self.status != MsgStatus::Rejected {
          log::warn!("{}", Self::PENDING_WARN);
        }
        self.body.set_none();
        self.status = MsgStatus::Pending;
      }
      MsgAction::Fulfilled => {
        if self.status != MsgStatus::Receiving {
          log::warn!("{}", Self::FULFILLED_WARN);
        }
        self.status = MsgStatus::Fulfilled;
      }
      MsgAction::Receiving(body) => {
        if self.status != MsgStatus::Pending {
          log::warn!("{}", Self::RECEIVING_WARN);
        }
        if let MsgBody::Text(str) = body {
          self.body.receive_text(str)
        }
        self.status = MsgStatus::Receiving;
      }
      MsgAction::Rejected => {
        if self.status != MsgStatus::Pending {
          log::warn!("{}", Self::REJECTED_WARN);
        }
        self.body.set_none();
        self.status = MsgStatus::Rejected;
      }
    }
  }

  pub fn body(&self) -> &MsgBody { &self.body }

  pub fn text(&self) -> Option<&String> {
    match &self.body {
      MsgBody::Text(s) => s.as_ref(),
      _ => None,
    }
  }

  pub fn image(&self) -> Option<&Image> {
    match &self.body {
      MsgBody::Image(i) => i.as_ref(),
      _ => None,
    }
  }

  pub fn status(&self) -> &MsgStatus { &self.status }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum MsgStatus {
  // Send message default status
  #[serde(rename = "pending")]
  Pending,
  // Message has response, but not finish
  #[serde(rename = "receiving")]
  Receiving,
  // Message don't have receive any response and trigger error(i.e timeout, invalid response)
  #[serde(rename = "rejected")]
  Rejected,
  // Once message has receive response, it will be fulfilled status, and can't update to other
  // status
  #[serde(rename = "fulfilled")]
  Fulfilled,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Default)]
pub enum MsgRole {
  #[default]
  #[serde(rename = "user")]
  User,
  #[serde(rename = "bot")]
  Bot(Uuid),
}

#[cfg(test)]
mod test {
  use super::*;
  use log::Level;
  extern crate testing_logger;

  #[test]
  fn msg_update() {
    testing_logger::setup();

    let mut msg = Msg::new(
      MsgRole::User,
      vec![MsgCont::text_init()],
      MsgMeta::default(),
    );
    assert_eq!(msg.cur_idx(), 0);
    msg.add_cont(MsgCont::text_init());
    assert_eq!(msg.cur_idx(), 1);
    assert_eq!(msg.cont_count(), 2);

    msg.switch_cont(0);
    assert_eq!(msg.cur_idx(), 0);

    msg.switch_cont(3);
    assert_eq!(msg.cur_idx(), 0);

    testing_logger::validate(|captured_logs| {
      assert_eq!(captured_logs.len(), 1);
      assert_eq!(
        captured_logs[0].body,
        "[polestar] switch message content error: index out of range: message's len is 2, but the index is 3"
      );
    });
  }

  #[test]
  fn msg_cont_action() {
    testing_logger::setup();

    let mut fulfilled_msg_cont = MsgCont::text_init();
    fulfilled_msg_cont.action(MsgAction::Receiving(MsgBody::Text(Some("123".to_owned()))));
    fulfilled_msg_cont.action(MsgAction::Fulfilled);
    let _fulfilled_msg = Msg::new(MsgRole::User, vec![fulfilled_msg_cont], MsgMeta::default());

    let mut rejected_msg_cont = MsgCont::text_init();
    rejected_msg_cont.action(MsgAction::Rejected);
    let _rejected_msg = Msg::new(MsgRole::User, vec![rejected_msg_cont], MsgMeta::default());

    let _pend_msg = Msg::new(
      MsgRole::User,
      vec![MsgCont::text_init()],
      MsgMeta::default(),
    );

    let mut fulfilled_msg_cont = MsgCont::text_init();
    fulfilled_msg_cont.action(MsgAction::Fulfilled);
    let _wrong_fulfilled_msg =
      Msg::new(MsgRole::User, vec![fulfilled_msg_cont], MsgMeta::default());

    let mut rejected_msg_cont = MsgCont::text_init();
    rejected_msg_cont.action(MsgAction::Receiving(MsgBody::Text(Some("123".to_owned()))));
    rejected_msg_cont.action(MsgAction::Rejected);
    let _wrong_rejected_msg = Msg::new(MsgRole::User, vec![rejected_msg_cont], MsgMeta::default());

    let mut pend_msg_cont = MsgCont::text_init();
    pend_msg_cont.action(MsgAction::Receiving(MsgBody::Text(Some("123".to_owned()))));
    pend_msg_cont.action(MsgAction::Pending);
    let _wrong_pend_msg = Msg::new(MsgRole::User, vec![pend_msg_cont], MsgMeta::default());

    let mut receiving_msg_cont = MsgCont::text_init();
    receiving_msg_cont.action(MsgAction::Rejected);
    receiving_msg_cont.action(MsgAction::Receiving(MsgBody::Text(Some("123".to_owned()))));
    let _wrong_receiving_msg =
      Msg::new(MsgRole::User, vec![receiving_msg_cont], MsgMeta::default());

    testing_logger::validate(|captured_logs| {
      assert_eq!(captured_logs.len(), 4);
      assert_eq!(captured_logs[0].body, MsgCont::FULFILLED_WARN,);
      assert_eq!(captured_logs[0].level, Level::Warn);

      assert_eq!(captured_logs[1].body, MsgCont::REJECTED_WARN,);
      assert_eq!(captured_logs[1].level, Level::Warn);

      assert_eq!(captured_logs[2].body, MsgCont::PENDING_WARN,);
      assert_eq!(captured_logs[2].level, Level::Warn);

      assert_eq!(captured_logs[3].body, MsgCont::RECEIVING_WARN,);
      assert_eq!(captured_logs[3].level, Level::Warn);
    });
  }
}
