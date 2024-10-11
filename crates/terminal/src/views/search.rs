mod actions;
mod completed;
mod search_input;

use self::actions::SearchAction;
use self::search_input::SearchInput;
use crate::api::{SearchMediaOptions, API};
use crate::component::Component;
use crossterm::event::{KeyCode, KeyEvent};
use protocol::channel::SearchMediaResponse;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::text::Text;
use ratatui::widgets::{Block, Paragraph};
use ratatui::Frame;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

#[derive(Clone)]
enum SearchState {
  Pending,
  Input,
  Searching,
  Completed(SearchMediaResponse),
  Error(String),
}

impl SearchState {
  fn is_pending(&self) -> bool {
    matches!(self, Self::Pending)
  }

  fn is_completed(&self) -> bool {
    matches!(self, Self::Completed(_)) || matches!(self, Self::Error(_))
  }
}

pub struct Search {
  api: API,
  input: SearchInput,
  state: SearchState,
  search_options: Option<SearchMediaOptions>,
  actions_rx: UnboundedReceiver<SearchAction>,
  actions_tx: UnboundedSender<SearchAction>,
  content: Option<Box<dyn Component<Action = SearchAction>>>,
}

impl Search {
  fn search(&self) {
    tokio::spawn({
      let search_options = self.search_options.clone().unwrap();
      let api = self.api.clone();

      let tx = self.actions_tx.clone();

      async move {
        let response = match api.media.search(&search_options).await {
          Ok(res) => res,
          Err(err) => {
            tx.send(SearchAction::Error(err.to_string()))?;

            return Err(err);
          }
        };

        tx.send(SearchAction::Completed(response))?;

        Ok::<_, anyhow::Error>(())
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

    if let Some(content) = self.content.as_mut() {
      if let Some(action) = content.on_key_event(key_event)? {
        self.actions_tx.send(action)?;
      }
    }

    match key_event.code {
      KeyCode::Char('s') if self.state.is_pending() => {
        self.actions_tx.send(SearchAction::StartEditing)?;
      }
      _ => {}
    };

    Ok(None)
  }

  fn update(&mut self, action: &Self::Action) -> anyhow::Result<Option<Self::Action>> {
    if let Self::Action::EndEditing | Self::Action::StartEditing = action {
      return Ok(Some(Self::Action::Render));
    }

    if self.actions_rx.is_empty() {
      return Ok(None);
    }

    while let Ok(action) = self.actions_rx.try_recv() {
      if let Some(action) = self.input.update(&action)? {
        self.actions_tx.send(action)?;
      }

      match action {
        SearchAction::Completed(res) => {
          self.state = SearchState::Completed(res.clone());
          self.content = Some(Box::new(completed::SearchCompeted::new(res)));
        }
        SearchAction::Cancelled => {
          self.state = SearchState::Pending;

          return Ok(Some(Self::Action::EndEditing));
        }
        SearchAction::Pending => {
          self.state = SearchState::Pending;
        }
        SearchAction::StartEditing => {
          self.state = SearchState::Input;
          self.input.start_editing();

          return Ok(Some(Self::Action::StartEditing));
        }
        SearchAction::Search(keyword) => {
          self.state = SearchState::Searching;
          self.search_options = Some(SearchMediaOptions { keyword, page: 1 });

          self.search();

          return Ok(Some(Self::Action::EndEditing));
        }
        SearchAction::Error(msg) => {
          self.state = SearchState::Error(msg);
        }
        _ => {}
      }
    }

    Ok(Some(Self::Action::Render))
  }

  fn render(&mut self, frame: &mut Frame, area: Rect) {
    let layout_chunks = Layout::vertical([Constraint::Length(3), Constraint::Min(1)]).split(area);

    self.input.render(frame, layout_chunks[0]);

    match &self.state {
      SearchState::Searching => {
        self.render_searching(frame, layout_chunks[1]);
      }
      SearchState::Completed(_) => {
        if let Some(content) = self.content.as_mut() {
          content.render(frame, layout_chunks[1]);
        }
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
      SearchState::Error(err) => {
        let text = Text::from(format!("Error: {}", err));

        let paragraph = Paragraph::new(text).block(Block::default()).centered();
        frame.render_widget(paragraph, layout_chunks[1]);
      }
    }
  }
}

impl Search {
  pub fn new(api: &API) -> Self {
    let (actions_tx, actions_rx) = unbounded_channel();

    Self {
      api: api.clone(),
      input: SearchInput::new_with_editing(true),
      state: SearchState::Input,
      search_options: None,
      content: None,
      actions_rx,
      actions_tx,
    }
  }
}
