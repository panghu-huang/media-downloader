use crate::component::Component;
use crate::state::CurrentlyView;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph};

#[derive(Default)]
pub struct StatusBar {
  currently_view: CurrentlyView,
}

impl Component for StatusBar {
  type Action = crate::actions::Action;

  fn update(&mut self, action: &Self::Action) -> anyhow::Result<Option<Self::Action>> {
    if let Self::Action::EnterSearchView = action {
      self.currently_view = CurrentlyView::Search;

      return Ok(Some(Self::Action::Render));
    }

    Ok(None)
  }

  fn render(&mut self, frame: &mut Frame, area: Rect) {
    let layout_chunks =
      Layout::horizontal([Constraint::Length(12), Constraint::Min(1)]).split(area);

    let block = Block::default().white().bg(Color::Indexed(27));

    let currently_view = match self.currently_view {
      CurrentlyView::Dashboard => "Dashboard",
      CurrentlyView::Search => "Search",
    }
    .to_string();

    let text = Text::from(currently_view);

    let paragraph = Paragraph::new(text).block(block).centered();

    frame.render_widget(paragraph, layout_chunks[0]);
  }
}
