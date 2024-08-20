mod actions;
mod search_input;

use self::actions::SearchAction;
use self::search_input::SearchInput;
use crate::component::Component;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::prelude::*;
use ratatui::widgets::*;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

#[derive(Copy, Clone, PartialEq, Eq)]
enum SearchState {
  Pending,
  Input,
  Searching,
  Completed,
}

pub struct Search {
  input: SearchInput,
  state: SearchState,
  actions_rx: UnboundedReceiver<SearchAction>,
  actions_tx: UnboundedSender<SearchAction>,
}

impl Search {
  fn search(&self, keyword: String) {
    tokio::spawn({
      let tx = self.actions_tx.clone();

      async move {
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        tx.send(SearchAction::Completed).unwrap();
      }
    });
  }

  fn render_searching(&self, frame: &mut Frame, area: Rect) {
    let text = Text::from("Searching...");

    let paragraph = Paragraph::new(text).block(Block::default()).centered();

    frame.render_widget(paragraph, area);
  }
}

impl Component for Search {
  type Action = crate::actions::Action;

  fn on_key_event(&mut self, key_event: KeyEvent) -> anyhow::Result<Option<Self::Action>> {
    if let Some(action) = self.input.on_key_event(key_event)? {
      self.actions_tx.send(action)?;
    }

    match key_event.code {
      KeyCode::Char('s') if self.state == SearchState::Pending => {
        self.actions_tx.send(SearchAction::StartEditing)?;
      }
      KeyCode::Char('c') if self.state == SearchState::Completed => {
        self.actions_tx.send(SearchAction::Pending)?;
      }
      _ => {}
    };

    Ok(None)
  }

  fn update(&mut self, action: &Self::Action) -> anyhow::Result<Option<Self::Action>> {
    while let Ok(action) = self.actions_rx.try_recv() {
      if let Some(action) = self.input.update(&action)? {
        self.actions_tx.send(action)?;
      }

      match action {
        SearchAction::Completed => {
          self.state = SearchState::Completed;
          return Ok(Some(Self::Action::Render));
        }
        SearchAction::Cancelled => {
          self.state = SearchState::Pending;
          return Ok(Some(Self::Action::EndEditing));
        }
        SearchAction::Pending => {
          self.state = SearchState::Pending;
          return Ok(Some(Self::Action::Render));
        }
        SearchAction::StartEditing => {
          self.state = SearchState::Input;
          self.input.start_editing();

          return Ok(Some(Self::Action::StartEditing));
        }
        SearchAction::Search(keyword) => {
          self.state = SearchState::Searching;
          self.search(keyword);
          return Ok(Some(Self::Action::EndEditing));
        }
        SearchAction::KeywordChanged => {
          return Ok(Some(Self::Action::Render));
        }
        SearchAction::Clear => {}
      }
    }

    if let Self::Action::EndEditing | Self::Action::StartEditing = action {
      return Ok(Some(Self::Action::Render));
    }

    Ok(None)
  }

  fn render(&self, frame: &mut Frame, area: Rect) {
    let layout_chunks = Layout::vertical([Constraint::Length(3), Constraint::Min(1)]).split(area);

    self.input.render(frame, layout_chunks[0]);

    match self.state {
      SearchState::Searching => {
        self.render_searching(frame, layout_chunks[1]);
      }
      SearchState::Completed => {
        let text = Text::from("Search completed. Press 'c' to continue");
        let paragraph = Paragraph::new(text).block(Block::default()).centered();
        frame.render_widget(paragraph, layout_chunks[1]);
      }
      SearchState::Pending => {
        let text = Text::from("Press 's' to start searching");
        let paragraph = Paragraph::new(text).block(Block::default()).centered();
        frame.render_widget(paragraph, layout_chunks[1]);
      }
      SearchState::Input => {
        let text = Text::from("Press 'Enter' to start searching. Press 'Esc' to cancel");

        let paragraph = Paragraph::new(text).block(Block::default()).centered();
        frame.render_widget(paragraph, layout_chunks[1]);
      }
    }
  }
}

impl Default for Search {
  fn default() -> Self {
    let (actions_tx, actions_rx) = unbounded_channel();

    Self {
      input: SearchInput::new_with_editing(true),
      state: SearchState::Input,
      actions_rx,
      actions_tx,
    }
  }
}
