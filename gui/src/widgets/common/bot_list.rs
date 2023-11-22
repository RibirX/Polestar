use std::rc::Rc;

use polestar_core::model::Bot;
use ribir::prelude::*;
use uuid::Uuid;

use crate::widgets::common::{w_avatar, InteractiveList};

#[derive(Declare)]
pub struct BotList {
  bots: Rc<Vec<Bot>>,
  #[declare(default = None)]
  selected_id: Option<Uuid>,
  #[declare(default = String::new())]
  filter: String,
}

type BotName = String;
impl BotList {
  pub fn set_filter(&mut self, filter: String) { self.filter = filter; }

  pub fn confirm(&mut self) {}

  pub fn move_up(&mut self) {
    let selected_id = self.next_selected(&self.selected_id, |this| this.get_bots().rev());
    self.selected_id = selected_id;
  }

  pub fn move_down(&mut self) {
    let selected_id = self.next_selected(&self.selected_id, |this| this.get_bots());
    self.selected_id = selected_id;
  }

  fn next_selected<'a, I>(
    &'a self,
    selected_id: &Option<Uuid>,
    it_creator: impl Fn(&'a BotList) -> I,
  ) -> Option<Uuid>
  where
    I: Iterator<Item = &'a Bot>,
  {
    let bot = {
      if let Some(selected_id) = selected_id {
        let mut it = it_creator(self).skip_while(|bot| bot.id() != selected_id);
        assert!(it.next().is_some());
        it.next().or_else(|| it_creator(self).next())
      } else {
        let mut it = it_creator(self);
        it.next()
      }
    };
    bot.map(|bot| *bot.id())
  }

  pub fn selected_bot(&self) -> Option<(Uuid, BotName)> {
    self.selected_id.as_ref().and_then(|id| {
      self
        .bots
        .iter()
        .find(|bot| bot.id() == id)
        .map(|bot| (*bot.id(), bot.name().to_string()))
    })
  }

  fn select(&mut self, bot_id: Uuid) { self.selected_id = Some(bot_id); }

  pub fn get_bots(&self) -> impl DoubleEndedIterator<Item = &Bot> {
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
          pipe!($this.filter.clone())
            .value_chain(|s| s.distinct_until_changed().box_it())
            .map(move |_| {
              let selected_id = { $this.get_bots().next().map(|bot| *bot.id()) };
              $this.silent().selected_id = selected_id;
              $this.get_bots().map(|bot| {
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
            })
        }
      }
    }
  }
}
