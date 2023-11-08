use std::time::Duration;

use ribir::prelude::*;

#[derive(Declare)]
pub struct Tooltip {
  content: String,
}

impl Compose for Tooltip {
  fn compose(this: impl StateWriter<Value = Self>) -> impl WidgetBuilder {
    fn_widget! {
      let mut text = @Text { text: $this.content.to_owned() };
      let appear_animate = @Animate {
        state: map_writer!($text.transform),
        from: Transform::translation(0., -200.),
        transition: Transition {
          duration: Duration::from_millis(100),
          easing: easing::EASE_IN,
        }.delay(Duration::from_secs(1)).box_it(),
      };
      let disappear_animate = @Animate {
        state: map_writer!($text.opacity),
        from: 1.,
        transition: Transition {
          duration: Duration::from_millis(100),
          easing: easing::EASE_OUT,
        }.delay(Duration::from_secs(1)).box_it(),
      };
      // TODO: remove animate, use `keep alive` widget
      watch!(!$appear_animate.is_running())
        .distinct_until_changed()
        .subscribe(move |is_finished| {
          if is_finished {
            $text.write().opacity = 0.;
            disappear_animate.run();
          }
        });
      @$text {
        padding: EdgeInsets::all(8.),
        on_mounted: move |_| {
          appear_animate.run();
        }
      }
    }
  }
}
