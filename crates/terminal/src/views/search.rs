mod actions;
mod search_input;

use self::actions::SearchAction;
use self::search_input::SearchInput;
use crate::component::Component;
use crossterm::event::{KeyCode, KeyEvent};
use protocol::channel::{MediaMetadata, SearchMediaResponse};
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::symbols::border;
use ratatui::text::{Line, Text};
use ratatui::widgets::block::Title;
use ratatui::widgets::{Block, Paragraph, Row, Table, TableState};
use ratatui::Frame;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

#[derive(Clone)]
enum SearchState {
  Pending,
  Input,
  Searching,
  Completed(SearchMediaResponse),
}

impl SearchState {
  fn is_pending(&self) -> bool {
    matches!(self, Self::Pending)
  }

  fn is_completed(&self) -> bool {
    matches!(self, Self::Completed(_))
  }
}

pub struct Search {
  input: SearchInput,
  state: SearchState,
  table_state: TableState,
  actions_rx: UnboundedReceiver<SearchAction>,
  actions_tx: UnboundedSender<SearchAction>,
}

impl Search {
  fn search(&self, keyword: String) {
    tokio::spawn({
      let tx = self.actions_tx.clone();

      async move {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        let response = SearchMediaResponse {
          total: 3,
          page: 1,
          page_size: 20,
          items: vec![
            MediaMetadata {
              name: "Title".to_string(),
              release_year: 2021,
              channel: "Channel".to_string(),
              id: "id".to_string(),
              poster_url: "".to_string(),
              description: "description".to_string(),
            },
            MediaMetadata {
              name: "海贼王".to_string(),
              release_year: 2021,
              channel: "Channel".to_string(),
              id: "id".to_string(),
              poster_url: "".to_string(),
              description: "descriptiondescriptiondescription".to_string(),
            },
            MediaMetadata {
              name: "Long description abcd".to_string(),
              release_year: 2021,
              channel: "Channel".to_string(),
              id: "id".to_string(),
              poster_url: "".to_string(),
              description: "descriptiondescriptiondescriptiondescription".to_string(),
            },
          ],
        };

        tx.send(SearchAction::Completed(response)).unwrap();
      }
    });
  }

  fn render_searching(&self, frame: &mut Frame, area: Rect) {
    let text = Text::from("Searching...");

    let paragraph = Paragraph::new(text).block(Block::default()).centered();

    frame.render_widget(paragraph, area);
  }

  fn render_completed(&mut self, frame: &mut Frame, area: Rect) {
    let mut rows = vec![];

    let SearchState::Completed(res) = &self.state else {
      return;
    };

    // Render tip & total count of media

    let total_page = (res.total as f32 / res.page_size as f32).ceil() as u32;

    let page_line = Line::from(vec![
      "Total pages found: ".into(),
      format!("{}.", total_page).bold().red(),
      " <Left / H> ".bold().green(),
      "to previous page".into(),
      " <Right / L> ".bold().green(),
      "to next page".into(),
    ]);

    let page_info = Title::from(page_line)
      .alignment(Alignment::Center)
      .position(ratatui::widgets::block::Position::Bottom);

    let block = Block::bordered()
      .title(
        Title::from(format!(
          " Total media found: {}. Press 'c' to clear ",
          res.total
        ))
        .alignment(Alignment::Center),
      )
      .title(page_info)
      .border_set(border::ROUNDED);

    // Render media list
    for (idx, item) in res.items.clone().iter().enumerate() {
      rows.push(
        Row::new(vec![
          (idx + 1).to_string().into(),
          item.name.clone().bold(),
          item.release_year.to_string().into(),
          item.description.clone().into(),
          item.channel.clone().into(),
        ])
        .height(2),
      );
    }

    let widths = [
      Constraint::Length(3),
      Constraint::Length(28),
      Constraint::Length(8),
      Constraint::Min(2),
      Constraint::Length(10),
    ];

    let selected_style = Style::default().bg(Color::Indexed(127));

    let table = Table::new(rows, widths)
      .block(block)
      .highlight_style(selected_style);

    frame.render_stateful_widget(table, area, &mut self.table_state);
  }
}

impl Component for Search {
  type Action = crate::actions::Action;

  fn on_key_event(&mut self, key_event: KeyEvent) -> anyhow::Result<Option<Self::Action>> {
    if let Some(action) = self.input.on_key_event(key_event)? {
      self.actions_tx.send(action)?;
    }

    match key_event.code {
      KeyCode::Char('s') if self.state.is_pending() => {
        self.actions_tx.send(SearchAction::StartEditing)?;
      }
      KeyCode::Char('c') if self.state.is_completed() => {
        self.actions_tx.send(SearchAction::Pending)?;
      }
      KeyCode::Down | KeyCode::Char('j') => {
        if let SearchState::Completed(res) = &self.state {
          let current_index = self.table_state.selected().unwrap_or(0);

          if current_index == res.items.len() - 1 {
            self.table_state.select_first();
          } else {
            self.table_state.select_next();
          }

          return Ok(Some(Self::Action::Render));
        }
      }
      KeyCode::Up | KeyCode::Char('k') if self.state.is_completed() => {
        let current_index = self.table_state.selected().unwrap_or(0);

        if current_index == 0 {
          self.table_state.select_last();
        } else {
          self.table_state.select_previous();
        }

        return Ok(Some(Self::Action::Render));
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
          self.table_state.select_first();
          self.state = SearchState::Completed(res);
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
          self.search(keyword);

          return Ok(Some(Self::Action::EndEditing));
        }
        SearchAction::KeywordChanged => {}
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
        self.render_completed(frame, layout_chunks[1]);
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
      table_state: TableState::default().with_selected(0),
      actions_rx,
      actions_tx,
    }
  }
}
