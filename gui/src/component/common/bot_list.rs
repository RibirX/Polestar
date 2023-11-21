use std::rc::Rc;

use polestar_core::model::Bot;
use ribir::prelude::*;
use uuid::Uuid;

use crate::component::common::{w_avatar, InteractiveList};

#[derive(Declare)]
pub struct BotList {
  bots: Rc<Vec<Bot>>,
  #[declare(default = None)]
  selected_id: Option<Uuid>,
  #[declare(default = String::new())]
  filter: String,
}

impl BotList {
  pub fn set_filter(&mut self, filter: String) { self.filter = filter; }

  pub fn confirm(&mut self) {}

  pub fn move_up(&mut self) {
    if let Some(selected_id) = self.selected_id {
      let idx = self.get_bots().position(|bot| bot.id() == &selected_id);

      if let Some(idx) = idx {
        if idx > 0 {
          self.selected_id = self.bots.iter().nth(idx - 1).map(|bot| bot.id().clone());
        } else {
          self.selected_id = self.bots.last().map(|bot| bot.id().clone());
        }
      }
    } else {
      self.selected_id = self.bots.first().map(|bot| bot.id().clone());
    }
  }

  pub fn move_down(&mut self) {
    if let Some(selected_id) = self.selected_id {
      let idx = self.get_bots().position(|bot| bot.id() == &selected_id);

      if let Some(idx) = idx {
        if idx < self.bots.len() - 1 {
          self.selected_id = self.bots.iter().nth(idx + 1).map(|bot| bot.id().clone());
        } else {
          self.selected_id = self.bots.first().map(|bot| bot.id().clone());
        }
      }
    } else {
      self.selected_id = self.bots.first().map(|bot| bot.id().clone());
    }
  }

  fn select(&mut self, bot_id: Uuid) { self.selected_id = Some(bot_id); }

  fn get_bots(&self) -> impl Iterator<Item = &Bot> {
    self
      .bots
      .iter()
      .filter(|bot| self.filter.is_empty() || bot.name().contains(&self.filter))
  }
}

impl Compose for BotList {
  fn compose(this: impl StateWriter<Value = Self>) -> impl WidgetBuilder {
    fn_widget! {
      let list = @InteractiveList {
        active: pipe!($this.selected_id).map(move |selected_id| {
          if let Some(selected_id) = selected_id {
            $this.get_bots().position(|bot| bot.id() == &selected_id).unwrap_or(0)
          } else {
            0
          }
        }).value_chain(|s| s.distinct_until_changed().box_it()),
      };

      @$list {
        @ {
          $this
            .get_bots()
            .map(|bot| {
              let bot_id = *bot.id();
              @ListItem {
                on_tap: move |_| {
                  $this.write().select(bot_id);
                },
                @Leading {
                  @ {
                    CustomEdgeWidget(
                      w_avatar(bot.avatar()).widget_build(ctx!())
                    )
                  }
                }
                @HeadlineText(Label::new(bot.name().to_owned()))
                @SupportingText(Label::new(bot.desc().map_or(String::new(), |s| s.to_owned())))
              }
            }).collect::<Vec<_>>()
        }
      }
    }
  }
}
