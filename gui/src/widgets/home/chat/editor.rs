use crate::widgets::common::BotList;
use polestar_core::{
  model::{Bot, Channel, Msg, MsgAction, MsgBody, MsgCont, MsgMeta, MsgRole},
  service::open_ai::mock_stream_string,
};
use ribir::prelude::*;
use std::ops::Range;
use std::rc::Rc;
use uuid::Uuid;

use crate::{style::CULTURED_F4F4F4_FF, theme::polestar_svg, widgets::common::IconButton};

pub fn w_editor(
  channel: impl StateWriter<Value = Channel>,
  bots: Rc<Vec<Bot>>,
  def_bot_id: Uuid,
) -> impl WidgetBuilder {
  fn_widget! {
    let mut bots = @BotList { bots, visible: false };
    let ignore_pointer = @IgnorePointer { ignore: false };
    let mut text_area = @MessageEditor {};
    let channel2 = channel.clone_writer();
    let send_icon = @IconButton {
      on_tap: move |_| {
        send_msg(&mut *$text_area.write(), channel.clone_writer(), def_bot_id);
      },
      @ { polestar_svg::SEND }
    };
    watch!($text_area.bot_hint())
      .distinct_until_changed()
      .subscribe(move |hint| {
        if let Some(mut hint) = hint {
          $bots.write().set_filter(hint.split_off(1));
          $bots.write().visible = $bots.get_bots().count() > 0;
        } else {
          $bots.write().visible = false;
        }
      });
    @Column {
      on_key_down_capture: move |e| {
        if $bots.visible {
          let mut stop_propagation = true;
          match e.key() {
            VirtualKey::Named(NamedKey::Escape) => $bots.write().visible = false,
            VirtualKey::Named(NamedKey::ArrowUp) => $bots.write().move_up(),
            VirtualKey::Named(NamedKey::ArrowDown) => $bots.write().move_down(),
            _ => stop_propagation = false,
          }
          if stop_propagation {
            e.stop_propagation();
          }
        }
      },
      on_chars_capture: move |e| {
        if e.chars == "\r" || e.chars == "\n" {
          if $bots.visible {
            select_bot(&mut *$text_area.write(), &$bots);
            e.stop_propagation();
          } else if !e.with_shift_key() {
            send_msg(&mut *$text_area.write(), channel2.clone_writer(), def_bot_id);
            e.stop_propagation();
          }
        }
      },
      @$bots {
        on_tap: move |_| {
          select_bot(&mut $text_area.write(), &$bots);
          $text_area.request_focus();
        }
      }
      @$ignore_pointer {
        @ConstrainedBox {
          clamp: BoxClamp {
            min: Size::new(f32::INFINITY, 48.),
            max: Size::new(f32::INFINITY, 114.),
          },
          padding: EdgeInsets::new(0., 11., 11., 11.),
          @Row {
            padding: EdgeInsets::all(10.),
            background: Color::from_u32(CULTURED_F4F4F4_FF),
            border_radius: Radius::all(8.),
            @Expanded {
              flex: 1.,
              @ $text_area {
                @ { Placeholder::new("Type a message") }
              }
            }
            @ { send_icon }
          }
        }
      }
    }
  }
}

fn send_msg(
  text_area: &mut MessageEditor,
  channel: impl StateWriter<Value = Channel>,
  def_bot_id: Uuid,
) {
  let text = text_area.display_text();
  let mut cont = MsgCont::text_init();
  cont.action(MsgAction::Receiving(MsgBody::Text(Some(text))));
  let msg = Msg::new(MsgRole::User, vec![cont], MsgMeta::default());
  channel.write().add_msg(msg);
  text_area.reset();
  let bots = text_area.edit_message.related_bot();
  let bot_id = bots.last().map_or(def_bot_id, |id| *id);
  let mut cont = MsgCont::text_init();
  cont.action(MsgAction::Receiving(MsgBody::Text(Some(
    "Hello, I'm Ribir!".to_string(),
  ))));
  let msg = Msg::new(MsgRole::Bot(bot_id), vec![cont], MsgMeta::default());
  let id = *msg.id();
  channel.write().add_msg(msg);

  if let Some(msg) = channel.write().msg_mut(&id) {
    let cur_cont = msg.cur_cont_mut();
    mock_stream_string("", |delta| {
      cur_cont.action(MsgAction::Receiving(MsgBody::Text(Some(delta))));
    });
  }
}

fn select_bot(text_area: &mut MessageEditor, bots: &BotList) {
  let hint = text_area.bot_hint();
  if let Some(hint) = hint {
    if let Some((uuid, name)) = bots.selected_bot() {
      let end = text_area.caret.cluster();
      text_area.delete(end - hint.len()..end);
      text_area.insert_bot(uuid, name)
    }
  }
}

enum MessageFragment {
  Text(String),
  Bot { uuid: Uuid, name: String },
}

#[derive(Clone, Copy)]
enum MoveDirection {
  Left,
  Right,
}

impl MessageFragment {
  fn display_text(&self) -> String {
    match self {
      MessageFragment::Text(txt) => txt.clone(),
      MessageFragment::Bot { name, .. } => format!("@ {} ", name),
    }
  }

  fn is_text(&self) -> bool { matches!(self, MessageFragment::Text(_)) }

  fn is_bot(&self) -> bool { matches!(self, MessageFragment::Bot { .. }) }
}

struct EditedMessage {
  fragments: Vec<MessageFragment>,
}

impl Default for EditedMessage {
  fn default() -> Self {
    Self {
      fragments: vec![MessageFragment::Text(String::default())],
    }
  }
}

impl EditedMessage {
  fn display_message(&self) -> String {
    let mut msg = String::new();
    self
      .fragments
      .iter()
      .for_each(|f| msg.push_str(&f.display_text()));
    msg
  }

  fn message_content(&self) -> String {
    let mut msg = String::new();
    self
      .fragments
      .iter()
      .filter(|f| matches!(f, MessageFragment::Text(_)))
      .for_each(|f| match f {
        MessageFragment::Text(txt) => msg.push_str(&txt),
        MessageFragment::Bot { .. } => unreachable!(),
      });
    msg
  }

  fn related_bot(&self) -> Vec<Uuid> {
    let mut uuids = vec![];
    self
      .fragments
      .iter()
      .filter(|f| matches!(*f, &MessageFragment::Bot { .. }))
      .for_each(|f| match f {
        MessageFragment::Text(_) => unreachable!(),
        MessageFragment::Bot { uuid, .. } => uuids.push(*uuid),
      });
    uuids
  }

  fn validate_cursor_position(&self, cursor: usize, dir: MoveDirection) -> usize {
    let mut acc = 0;
    for fragment in self.fragments.iter() {
      let len = match fragment {
        MessageFragment::Text(txt) => txt.len(),
        MessageFragment::Bot { .. } => fragment.display_text().len(),
      };

      acc += len;
      let range = acc - len..acc;
      if range.contains(&cursor) {
        match fragment {
          MessageFragment::Text(_) => return cursor,
          MessageFragment::Bot { .. } => {
            let mut position = cursor;
            if position != range.start && position != range.end {
              position = match dir {
                MoveDirection::Left => range.start,
                MoveDirection::Right => range.end,
              };
            }

            return position;
          }
        }
      }
    }

    acc
  }

  fn delete(&mut self, selected: Range<usize>) -> usize {
    if selected.is_empty() {
      return selected.start;
    }
    let mut acc = 0;
    let mut cursor = selected.start;
    let mut deletes_fragment = vec![];
    let mut idx = 0;
    while idx < self.fragments.len() && acc < selected.end {
      let len = match &self.fragments[idx] {
        MessageFragment::Text(txt) => txt.len(),
        fragment @ MessageFragment::Bot { .. } => fragment.display_text().len(),
      };

      acc += len;
      let range: Range<usize> = acc - len..acc;
      if selected.start < range.end && range.start < selected.end {
        if selected.start < range.start && range.end < selected.end {
          deletes_fragment.push(idx);
        } else {
          match &mut self.fragments[idx] {
            MessageFragment::Text(txt) => {
              let start = selected.start.max(range.start) - range.start;
              let end = selected.end.min(range.end) - range.start;
              txt.replace_range(start..end, "");
            }
            MessageFragment::Bot { .. } => {
              cursor = cursor.min(range.start);
              deletes_fragment.push(idx);
            }
          }
        }
      }
      idx += 1;
    }

    deletes_fragment.into_iter().rev().for_each(|i| {
      self.fragments.remove(i);
    });

    if self.fragments.is_empty() {
      self
        .fragments
        .push(MessageFragment::Text(String::default()));
    }

    let (idx, _) = self.position(cursor);
    self.try_merge_text(idx, 2);

    cursor
  }

  fn insert_str(&mut self, mut cursor: usize, s: &str) -> usize {
    let (mut i, mut offset) = self.position(cursor);
    if self.fragments[i].is_bot() {
      self
        .fragments
        .insert(i + 1, MessageFragment::Text(String::default()));
      cursor = cursor - offset + self.fragments[i].display_text().len();
      offset = 0;
      i = i + 1;
    }

    match &mut self.fragments[i] {
      MessageFragment::Text(txt) => {
        txt.insert_str(offset, s);
      }
      MessageFragment::Bot { .. } => unreachable!(),
    }

    self.try_merge_text(i, 2);

    return cursor + s.len();
  }

  pub fn insert_bot(&mut self, mut cursor: usize, uuid: Uuid, name: String) -> usize {
    let (mut idx, offset) = self.position(cursor);
    match &mut self.fragments[idx] {
      MessageFragment::Text(txt) => {
        let new_text = txt.split_off(offset);
        if !new_text.is_empty() {
          self
            .fragments
            .insert(idx + 1, MessageFragment::Text(new_text));
        }
      }
      fragment @ MessageFragment::Bot { .. } => {
        cursor = cursor - offset + fragment.display_text().len();
      }
    };

    idx += 1;
    let bot = MessageFragment::Bot { uuid: uuid, name: name };
    cursor += bot.display_text().len();
    self.fragments.insert(idx, bot);
    cursor
  }

  fn try_merge_text(&mut self, from: usize, len: usize) {
    assert!(self.fragments.len() > 0 && len > 0);
    let to = (from + len - 1).min(self.fragments.len() - 1);
    let mut last_text = None;

    for idx in (from..=to).rev() {
      if self.fragments[idx].is_text() {
        if last_text == None {
          last_text = Some(idx);
        }
      } else {
        if let Some(last_text) = last_text {
          self.merge_text(idx + 1, last_text);
        }
        last_text = None;
      }
    }
    if let Some(last_text) = last_text {
      self.merge_text(from, last_text);
    }
  }

  fn merge_text(&mut self, from: usize, to: usize) {
    {
      if to - from > 0 {
        let mut txt = String::default();
        for i in from..=to {
          if let MessageFragment::Text(t) = &self.fragments[i] {
            txt.push_str(t);
          }
        }
        self.fragments[from] = MessageFragment::Text(txt);
        let _: Vec<_> = self.fragments.drain(from + 1..=to).collect();
      }
    };
  }

  fn position(&self, mut cursor: usize) -> (usize, usize) {
    assert!(!self.fragments.is_empty());
    let mut max_offset = 0;
    for idx in 0..self.fragments.len() {
      max_offset = match &self.fragments[idx] {
        MessageFragment::Text(txt) => txt.len(),
        fragment @ MessageFragment::Bot { .. } => fragment.display_text().len(),
      };

      if max_offset >= cursor {
        return (idx, cursor);
      }
      cursor -= max_offset;
    }
    (self.fragments.len() - 1, max_offset)
  }
}

#[derive(Declare)]
pub struct MessageEditor {
  #[declare(skip)]
  edit_message: EditedMessage,
  #[declare(skip)]
  caret: CaretState,
}

impl MessageEditor {
  pub fn reset(&mut self) {
    self.edit_message = EditedMessage::default();
    self.caret = CaretState::default();
  }

  pub fn display_text(&self) -> String { self.edit_message.display_message() }

  pub fn insert_bot(&mut self, uuid: Uuid, name: String) {
    let mut cursor = self.caret.select_range().start;
    cursor = self.edit_message.insert_bot(cursor, uuid, name);
    self.caret = CaretState::Caret(CaretPosition { cluster: cursor, position: None });
  }

  pub fn bot_hint(&self) -> Option<String> {
    if !matches!(self.caret, CaretState::Caret(_)) {
      return None;
    }
    let (idx, offset) = self.edit_message.position(self.caret.cluster());
    match self.edit_message.fragments.get(idx) {
      Some(MessageFragment::Text(txt)) => {
        let substr = &txt[0..offset];
        substr.rfind('@').map(|idx| substr[idx..].to_string())
      }
      _ => None,
    }
  }

  pub fn delete(&mut self, rg: Range<usize>) {
    if rg.is_empty() {
      return;
    }
    let cluster = self.edit_message.delete(rg);
    self.caret = CaretState::Caret(CaretPosition { cluster, position: None });
  }

  pub fn insert_str(&mut self, txt: &str) {
    let rg = self.caret.select_range();
    self.delete(rg);
    let mut cluster = self.caret.cluster();
    cluster = self.edit_message.insert_str(cluster, &txt);
    self.caret = CaretPosition { cluster, position: None }.into();
  }

  fn update_caret_by_move(&mut self, mut caret: CaretState, dir: MoveDirection) {
    match &mut caret {
      CaretState::Caret(pos) | CaretState::Select(_, pos) | CaretState::Selecting(_, pos) => {
        let cluster = self.edit_message.validate_cursor_position(pos.cluster, dir);
        if cluster != pos.cluster {
          *pos = CaretPosition { cluster, position: None };
        }
      }
    }
    self.caret = caret;
  }

  fn set_caret_with_check(&mut self, caret: CaretState) {
    let mut caret = caret.clone();
    match &mut caret {
      CaretState::Caret(pos) => {
        let cluster = self
          .edit_message
          .validate_cursor_position(pos.cluster, MoveDirection::Right);
        if cluster != pos.cluster {
          *pos = CaretPosition { cluster, position: None };
        }
      }
      CaretState::Select(start, end) | CaretState::Selecting(start, end) => {
        let is_reverse = start.cluster > end.cluster;
        let dirs = if is_reverse {
          (MoveDirection::Right, MoveDirection::Left)
        } else {
          (MoveDirection::Left, MoveDirection::Right)
        };
        let mut cluster = self
          .edit_message
          .validate_cursor_position(start.cluster, dirs.0);
        if cluster != end.cluster {
          *start = CaretPosition { cluster, position: None };
        }
        cluster = self
          .edit_message
          .validate_cursor_position(end.cluster, dirs.1);
        if cluster != end.cluster {
          *end = CaretPosition { cluster, position: None };
        }
      }
    }
    self.caret = caret;
  }
}

impl ComposeChild for MessageEditor {
  type Child = Option<State<Placeholder>>;
  fn compose_child(this: impl StateWriter<Value = Self>, child: Self::Child) -> impl WidgetBuilder {
    fn_widget! {
      let text_area = @TextArea { rows: None, cols: None };
      $text_area.write().set_text_with_caret(&$this.edit_message.display_message(), $this.caret);
      watch!(($this.edit_message.display_message(), $this.caret))
        .distinct_until_changed()
        .subscribe(move |(txt, caret)| {
          $text_area.write().set_text_with_caret(&txt, caret);
        });

      @ $text_area  {
        on_key_down_capture: move |e| {
          let _capture_hint = || $this.write();
          if deal_delete(&this, e) || deal_copy_paste(&this, e) {
            e.stop_propagation();
          }
        },
        on_key_down: move |e| {
          let caret = $text_area.caret();
          match e.key() {
            VirtualKey::Named(NamedKey::ArrowLeft) |
            VirtualKey::Named(NamedKey::ArrowUp) |
            VirtualKey::Named(NamedKey::Home) => {
              $this.write().update_caret_by_move(caret, MoveDirection::Left);
            }
            VirtualKey::Named(NamedKey::ArrowRight) |
            VirtualKey::Named(NamedKey::ArrowDown) |
            VirtualKey::Named(NamedKey::End) => {
              $this.write().update_caret_by_move(caret, MoveDirection::Right);
            }
            VirtualKey::Named(NamedKey::Process) => (),
            _ => $this.write().set_caret_with_check($text_area.caret()),
          }
        },
        on_double_tap: move |_| $this.write().set_caret_with_check($text_area.caret()),
        on_pointer_down: move |_| $this.write().set_caret_with_check($text_area.caret()),
        on_pointer_move:move |_| $this.write().set_caret_with_check($text_area.caret()),
        on_pointer_up: move |_| $this.write().set_caret_with_check($text_area.caret()),
        on_chars_capture: move |e| {
          e.stop_propagation();
          if e.with_command_key() {
            return;
          }
          let txt = e
            .chars
            .chars()
            .filter(|c| !c.is_control() || c.is_ascii_whitespace())
            .collect::<String>();

          $this.write().insert_str(&txt);
        },
        @ {child}
      }
    }
  }
}

fn deal_copy_paste(this: &impl StateWriter<Value = MessageEditor>, e: &KeyboardEvent) -> bool {
  let copy = |editor: &MessageEditor| {
    let rg = editor.caret.select_range();
    if rg.is_empty() {
      return false;
    }
    let txt = editor.display_text()[rg.clone()].to_string();
    let clipboard = AppCtx::clipboard();
    let _ = clipboard.borrow_mut().clear();
    let _ = clipboard.borrow_mut().write_text(&txt);
    true
  };
  match e.key_code() {
    PhysicalKey::Code(KeyCode::KeyV) => {
      if let Ok(txt) = AppCtx::clipboard().borrow_mut().read_text() {
        this.write().insert_str(&txt);
      }
      true
    }
    PhysicalKey::Code(KeyCode::KeyC) => copy(&this.read()) || true,
    PhysicalKey::Code(KeyCode::KeyX) => {
      if copy(&this.read()) {
        let rg = this.read().caret.select_range();
        this.write().delete(rg);
      }
      true
    }
    _ => false,
  }
}

fn deal_delete(this: &impl StateWriter<Value = MessageEditor>, e: &KeyboardEvent) -> bool {
  match e.key() {
    VirtualKey::Named(NamedKey::Backspace) => {
      let mut rg = this.read().caret.select_range();
      if rg.is_empty() {
        if let Some(c) = this.read().display_text()[..rg.start].chars().last() {
          rg.start -= c.len_utf8();
        }
      }

      this.write().delete(rg);
      true
    }
    VirtualKey::Named(NamedKey::Delete) => {
      let mut rg = this.read().caret.select_range();
      if rg.is_empty() {
        if let Some(c) = this.read().display_text()[rg.start..].chars().next() {
          rg.end += c.len_utf8();
        }
      }
      this.write().delete(rg);
      true
    }
    _ => false,
  }
}
#[cfg(test)]
mod tests {
  use ribir::widgets::input::{CaretPosition, CaretState};
  use uuid::Uuid;

  use crate::widgets::home::chat::editor::{
    EditedMessage, MessageEditor, MessageFragment, MoveDirection,
  };

  #[test]
  fn message_editor_caret() {
    let mut editor = MessageEditor {
      edit_message: EditedMessage {
        fragments: vec![
          MessageFragment::Text("Hello ".to_string()),
          MessageFragment::Bot {
            uuid: Uuid::parse_str("a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8").unwrap(),
            name: "Ribir".to_string(),
          },
          MessageFragment::Text("nice to see you!".to_string()),
        ],
      },
      caret: CaretState::default(),
    };

    assert_eq!(editor.display_text(), "Hello @ Ribir nice to see you!");
    editor.set_caret_with_check(CaretState::Caret(CaretPosition {
      cluster: 6,
      position: None,
    }));
    assert_eq!(
      CaretState::Caret(CaretPosition { cluster: 6, position: None }),
      editor.caret
    );

    editor.update_caret_by_move(
      CaretState::Caret(CaretPosition { cluster: 7, position: None }),
      MoveDirection::Right,
    );
    assert_eq!(
      CaretState::Caret(CaretPosition { cluster: 14, position: None }),
      editor.caret
    );

    editor.update_caret_by_move(
      CaretState::Caret(CaretPosition { cluster: 13, position: None }),
      MoveDirection::Left,
    );
    assert_eq!(
      CaretState::Caret(CaretPosition { cluster: 6, position: None }),
      editor.caret
    );

    editor.set_caret_with_check(CaretState::Caret(CaretPosition {
      cluster: 8,
      position: None,
    }));
    assert_eq!(
      CaretState::Caret(CaretPosition { cluster: 14, position: None }),
      editor.caret
    );
  }

  #[test]
  fn message_edit() {
    let mut messages = EditedMessage {
      fragments: vec![MessageFragment::Text("Hello nice to see you!".to_string())],
    };

    let cursor = messages.insert_bot(
      6,
      Uuid::parse_str("a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8").unwrap(),
      "Ribir".to_string(),
    );

    assert_eq!(cursor, 14);

    messages.insert_str(cursor, ", ");

    assert_eq!(
      messages.display_message(),
      "Hello @ Ribir , nice to see you!"
    );
    assert_eq!(messages.message_content(), "Hello , nice to see you!");
    assert_eq!(messages.fragments.len(), 3);

    messages.delete(5..15);
    assert_eq!(messages.message_content(), "Hello nice to see you!");
    assert_eq!(messages.fragments.len(), 1);

    messages.delete(0..25);
    assert_eq!(messages.display_message(), "");
    assert_eq!(messages.fragments.len(), 1);
  }
}
