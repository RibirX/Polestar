use std::rc::Rc;

use polestar_core::model::{Bot, BotId};
use ribir::prelude::*;

use crate::{
  style::WHITE,
  widgets::common::{w_avatar, InteractiveList},
};

#[derive(Declare)]
pub struct BotList {
  bots: Rc<Vec<Bot>>,
  #[declare(default = None)]
  selected_id: Option<BotId>,
  #[declare(default = String::new())]
  filter: String,
}

type BotName = String;
impl BotList {
  pub fn set_filter(&mut self, filter: String, reset_select: bool) {
    self.filter = filter;
    if reset_select
      || !self
        .selected_bot()
        .map(|(_, name)| name.contains(&self.filter))
        .unwrap_or_default()
    {
      self.init_select(true);
    }
  }

  pub fn move_up(&mut self) {
    let selected_id = self.next_selected(&self.selected_id, |this| this.get_bots().rev());
    self.set_selected_bot(selected_id)
  }

  pub fn move_down(&mut self) {
    let selected_id = self.next_selected(&self.selected_id, |this| this.get_bots());
    self.set_selected_bot(selected_id)
  }

  fn next_selected<'a, I>(
    &'a self,
    selected_id: &Option<BotId>,
    it_creator: impl Fn(&'a BotList) -> I,
  ) -> Option<BotId>
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
    bot.map(|bot| bot.id().clone())
  }

  pub fn selected_bot(&self) -> Option<(BotId, BotName)> {
    self.selected_id.as_ref().and_then(|id| {
      self
        .bots
        .iter()
        .find(|bot| bot.id() == id)
        .map(|bot| (bot.id().clone(), bot.name().to_string()))
    })
  }

  pub fn get_bots(&self) -> impl DoubleEndedIterator<Item = &Bot> {
    self
      .bots
      .iter()
      .filter(|bot| self.filter.is_empty() || bot.name().contains(&self.filter))
  }

  pub fn set_selected_bot(&mut self, bot_id: Option<BotId>) { self.selected_id = bot_id; }

  fn init_select(&mut self, force: bool) {
    if self.selected_id.is_some() && !force {
      return;
    }
    let selected_id = { self.get_bots().next().map(|bot| bot.id().clone()) };
    self.selected_id = selected_id;
  }
}

impl Compose for BotList {
  fn compose(this: impl StateWriter<Value = Self>) -> impl WidgetBuilder {
    fn_widget! {
      let list = @InteractiveList {
        active: pipe!($this.selected_id.clone()).map(move |selected_id| {
          if let Some(selected_id) = selected_id {
            $this.get_bots().position(|bot| bot.id() == &selected_id).unwrap_or(0)
          } else {
            0
          }
        }).value_chain(|s| s.distinct_until_changed().box_it()),
      };

      this.silent().init_select(false);
      @$list {
        @ {
          pipe!($this.filter.clone())
            .value_chain(|s| s.distinct_until_changed().box_it())
            .map(move |_| {
              $this.get_bots().map(|bot| {
                let bot_id = bot.id().clone();
                @ListItem {
                  on_tap: move |_| {
                    $this.write().set_selected_bot(Some(bot_id.clone()));
                  },
                  @Leading {
                    @ {
                      CustomEdgeWidget(
                        w_avatar(bot.avatar().clone()).widget_build(ctx!())
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
