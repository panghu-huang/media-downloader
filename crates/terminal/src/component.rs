use ratatui::crossterm::event::KeyEvent;
use ratatui::prelude::*;

pub trait Component<Action = ()> {
  fn on_key_event(&mut self, key_event: KeyEvent) -> anyhow::Result<Option<Action>> {
    let _ = key_event;
    Ok(None)
  }

  fn update(&mut self, action: Action) -> anyhow::Result<Option<Action>> {
    let _ = action;
    Ok(None)
  }

  fn render(&self, frame: &mut Frame, area: Rect);
}
