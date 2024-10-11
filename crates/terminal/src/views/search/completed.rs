use super::actions::SearchAction;
use crate::component::Component;
use crossterm::event::{KeyCode, KeyEvent};
use protocol::channel::SearchMediaResponse;
use ratatui::layout::{Alignment, Constraint, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::symbols::border;
use ratatui::text::Line;
use ratatui::widgets::block::Title;
use ratatui::widgets::{Block, Cell, Row, Table, TableState};
use ratatui::Frame;

pub struct SearchCompeted {
  table_state: TableState,
  data: SearchMediaResponse,
}

impl Component for SearchCompeted {
  type Action = SearchAction;

  fn on_key_event(&mut self, key_event: KeyEvent) -> anyhow::Result<Option<Self::Action>> {
    match key_event.code {
      KeyCode::Char('c') => {
        return Ok(Some(SearchAction::Pending));
      }
      KeyCode::Down | KeyCode::Char('j') => {
        let current_index = self.table_state.selected().unwrap_or(0);

        if current_index == self.data.items.len() - 1 {
          self.table_state.select_first();
        } else {
          self.table_state.select_next();
        }

        return Ok(Some(Self::Action::Render));
      }
      KeyCode::Up | KeyCode::Char('k') => {
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

  fn update(&mut self, _action: &Self::Action) -> anyhow::Result<Option<Self::Action>> {
    Ok(None)
  }

  fn render(&mut self, frame: &mut Frame, area: Rect) {
    let mut rows = vec![];

    // Render tip & total count of media
    let total_page = (self.data.total as f32 / self.data.page_size as f32).ceil() as u32;

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
          self.data.total
        ))
        .alignment(Alignment::Center),
      )
      .title(page_info)
      .border_set(border::ROUNDED);

    // Render media list
    for (idx, item) in self.data.items.clone().iter().enumerate() {
      rows.push(
        Row::new(vec![
          Cell::from(format!("\n{}", idx + 1)),
          Cell::from(format!("\n{}", item.name)),
          Cell::from(format!("\n{}", item.release_year)),
          Cell::from(format!("\n{}", item.description)),
          Cell::from(format!("\n{}", item.channel)),
        ])
        .height(3),
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

impl SearchCompeted {
  pub fn new(data: SearchMediaResponse) -> Self {
    Self {
      data,
      table_state: TableState::default().with_selected(0),
    }
  }
}
