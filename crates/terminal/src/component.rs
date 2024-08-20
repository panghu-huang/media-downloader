use ratatui::crossterm::event::KeyEvent;
use ratatui::prelude::*;

pub trait Component {
  type Action;

  fn on_key_event(&mut self, key_event: KeyEvent) -> anyhow::Result<Option<Self::Action>> {
    let _ = key_event;
    Ok(None)
  }

  fn update(&mut self, action: &Self::Action) -> anyhow::Result<Option<Self::Action>> {
    let _ = action;
    Ok(None)
  }

  fn render(&self, frame: &mut Frame, area: Rect);
}

pub type BoxedComponent<A> = Box<dyn Component<Action = A>>;
