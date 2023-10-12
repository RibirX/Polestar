use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Polestar channel is a chat room, user can talk to bot in channel.
#[derive(Debug, Builder, Default, Deserialize, Serialize, sqlx::FromRow)]
#[builder(name = "ChannelBuilder")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
pub struct Channel {
  // A unique id for channel
  #[builder(default = "Uuid::new_v4()")]
  id: Uuid,
  // A name for channel
  name: String,
  // A description for channel, it's optional
  desc: Option<String>,
  // A configuration for channel
  #[sqlx(json)]
  cfg: ChannelCfg,
}

impl Channel {
  pub fn id(&self) -> &Uuid { &self.id }

  pub fn name(&self) -> &str { &self.name }

  pub fn set_name(&mut self, name: String) { self.name = name; }

  pub fn desc(&self) -> Option<&str> { self.desc.as_deref() }

  pub fn set_desc(&mut self, desc: Option<String>) { self.desc = desc; }

  pub fn cfg_ref(&self) -> &ChannelCfg { &self.cfg }

  pub fn cfg_mut(&mut self) -> &mut ChannelCfg { &mut self.cfg }
}

/// ChannelMode is a enum type for channel's mode, it decided AI chat context
/// count.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum ChannelMode {
  #[default]
  Balanced,
  Performance,
}

/// ChannelKind is a enum type for channel's kind, it decided channel's usage.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum ChannelKind {
  #[default]
  Chat,
  QuickLauncher,
  Feedback,
}

#[derive(Default, Builder, Debug, Clone, Deserialize, Serialize)]
#[builder(name = "ChannelCfgBuilder")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug, Deserialize, Serialize))]
pub struct ChannelCfg {
  mode: ChannelMode,
  kind: ChannelKind,
  def_bot_id: Option<Uuid>,
}

impl ChannelCfg {
  pub fn mode(&self) -> &ChannelMode { &self.mode }

  pub fn kind(&self) -> &ChannelKind { &self.kind }

  pub fn def_bot_id(&self) -> Option<&Uuid> { self.def_bot_id.as_ref() }

  pub fn set_mode(&mut self, mode: ChannelMode) { self.mode = mode; }

  pub fn set_kind(&mut self, kind: ChannelKind) { self.kind = kind; }

  pub fn set_def_bot_id(&mut self, bot_id: Uuid) { self.def_bot_id = Some(bot_id); }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_channel() {
    let mut channel = ChannelBuilder::default()
      .id(Uuid::new_v4())
      .name("test")
      .desc("test channel")
      .build()
      .unwrap();
    println!("{:?}", channel);

    assert_eq!(channel.name(), "test");
    assert_eq!(channel.desc(), Some("test channel"));
    assert_eq!(channel.cfg_ref().mode(), &ChannelMode::Balanced);
    assert_eq!(channel.cfg_ref().kind(), &ChannelKind::Chat);
    assert_eq!(channel.cfg_ref().def_bot_id(), None);

    channel.cfg_mut().set_mode(ChannelMode::Performance);
    assert_eq!(channel.cfg_ref().mode(), &ChannelMode::Performance);

    channel.cfg_mut().set_kind(ChannelKind::Feedback);
    assert_eq!(channel.cfg_ref().kind(), &ChannelKind::Feedback);

    let id = Uuid::new_v4();
    channel.cfg_mut().set_def_bot_id(id);
    assert_eq!(channel.cfg_ref().def_bot_id(), Some(&id));
  }

  #[test]
  fn deserialize_channel() {
    let channel_str = "{\"id\": \"00000000-0000-0000-0000-000000000000\", \"name\": \"test\", \"desc\": \"test channel\", \"cfg\": {\"mode\": \"Balanced\", \"kind\": \"Chat\"}}";
    let channel = serde_json::from_str::<Channel>(channel_str).unwrap();
    println!("{:?}", channel);
    assert_eq!(channel.id(), &Uuid::nil());
    assert_eq!(channel.name(), "test");
    assert_eq!(channel.desc(), Some("test channel"));
    assert_eq!(channel.cfg_ref().mode(), &ChannelMode::Balanced);
    assert_eq!(channel.cfg_ref().kind(), &ChannelKind::Chat);
  }

  #[test]
  fn serialize_channel() {
    let channel = ChannelBuilder::default()
      .id(Uuid::nil())
      .name("test")
      .desc("test channel")
      .build()
      .unwrap();
    let channel_str = serde_json::to_string(&channel).unwrap();
    println!("{}", channel_str);
    assert_eq!(
      channel_str,
      "{\"id\":\"00000000-0000-0000-0000-000000000000\",\"name\":\"test\",\"desc\":\"test channel\",\"cfg\":{\"mode\":\"Balanced\",\"kind\":\"Chat\",\"def_bot_id\":null}}"
    );
  }
}
