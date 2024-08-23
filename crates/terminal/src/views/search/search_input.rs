use super::actions::SearchAction;
use crate::component::Component;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::prelude::*;
use ratatui::widgets::*;
use symbols::border;

pub struct SearchInput {
  editing: bool,
  inputs: Vec<char>,
}

impl SearchInput {
  pub fn start_editing(&mut self) {
    self.editing = true;
  }

  pub fn end_editing(&mut self) {
    self.editing = false;
  }
}

impl Component for SearchInput {
  type Action = SearchAction;

  fn on_key_event(&mut self, key_event: KeyEvent) -> anyhow::Result<Option<Self::Action>> {
    if self.editing {
      match key_event.code {
        KeyCode::Char(c) => {
          self.inputs.push(c);
        }
        KeyCode::Backspace => {
          self.inputs.pop();
        }
        KeyCode::Esc => {
          return Ok(Some(SearchAction::Cancelled));
        }
        KeyCode::Enter => {
          return Ok(Some(SearchAction::Search(self.inputs.iter().collect())));
        }
        _ => {}
      };

      return Ok(Some(Self::Action::KeywordChanged));
    }

    Ok(None)
  }

  fn update(&mut self, action: &Self::Action) -> anyhow::Result<Option<Self::Action>> {
    match action {
      SearchAction::StartEditing => {
        self.start_editing();
      }
      SearchAction::Completed(_)
      | SearchAction::Cancelled
      | SearchAction::Pending
      | SearchAction::Search(_) => {
        self.end_editing();
      }
      _ => {}
    };

    Ok(None)
  }

  fn render(&mut self, frame: &mut Frame, area: Rect) {
    let text = Text::from(self.inputs.iter().collect::<String>());

    let block = Block::bordered().title(" Search ");
    let block = if self.editing {
      block.border_set(border::DOUBLE)
    } else {
      block.border_set(border::ROUNDED)
    };

    let paragraph = Paragraph::new(text).block(block);

    frame.render_widget(paragraph, area);
  }
}

impl SearchInput {
  pub fn new_with_editing(editing: bool) -> Self {
    Self {
      inputs: vec![],
      editing,
    }
  }
}
