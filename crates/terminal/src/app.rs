use crate::actions::Action;
use crate::api::API;
use crate::component::BoxedComponent;
use crate::state::{AppState, CurrentlyView};
use crate::terminal::{Terminal, TerminalEvent};
use crate::views::dashboard::Dashboard;
use crate::views::search::Search;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::prelude::*;
use ratatui::widgets::*;
use std::io;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

const API_BASE_URL: &str = "http://192.168.3.4:5231";

pub struct App {
  should_quit: bool,
  state: AppState,
  api: API,
  //status_bar: StatusBar,
  currently_view: BoxedComponent<Action>,
  actions_rx: UnboundedReceiver<Action>,
  actions_tx: UnboundedSender<Action>,
}

impl App {
  pub fn new() -> Self {
    let (actions_tx, actions_rx) = unbounded_channel();
    let api = API::new(API_BASE_URL);

    Self {
      api,
      actions_rx,
      actions_tx,
      should_quit: false,
      state: AppState::default(),
      //status_bar: StatusBar::default(),
      currently_view: Box::new(Dashboard) as BoxedComponent<Action>,
    }
  }

  pub async fn run(&mut self) -> anyhow::Result<()> {
    let mut terminal = Terminal::new(4, 60)?;

    terminal.enter()?;

    self.actions_tx.send(Action::Render)?;

    loop {
      self.handle_event(&mut terminal).await?;
      self.handle_actions(&mut terminal)?;

      if self.should_quit {
        break;
      }
    }

    terminal.exit()?;

    Ok(())
  }

  async fn handle_event(&mut self, terminal: &mut Terminal) -> anyhow::Result<()> {
    let Some(event) = terminal.next_event().await else {
      return Ok(());
    };

    match event {
      TerminalEvent::KeyEvent(key_event) if key_event.kind == KeyEventKind::Press => {
        self.handke_key_event(key_event)?;
      }
      TerminalEvent::Quit => self.actions_tx.send(Action::Quit)?,
      TerminalEvent::Tick => self.actions_tx.send(Action::Tick)?,
      _ => {}
    }

    Ok(())
  }

  fn handle_actions(&mut self, terminal: &mut Terminal) -> anyhow::Result<()> {
    while let Ok(action) = self.actions_rx.try_recv() {
      match action {
        Action::Quit => {
          self.should_quit = true;
        }
        Action::Render => {
          self.render(terminal)?;
        }
        Action::EnterSearchView => {
          self.currently_view = Box::new(Search::new(&self.api));
          self.state.editing = true;

          self.actions_tx.send(Action::Render)?;
        }
        Action::StartEditing => {
          self.state.editing = true;
        }
        Action::EndEditing => {
          self.state.editing = false;
        }
        _ => {}
      };

      if let Some(action) = self.currently_view.update(&action)? {
        self.actions_tx.send(action)?;
      }
      //if let Some(action) = self.status_bar.update(&action)? {
      //  self.actions_tx.send(action)?;
      //}
    }

    Ok(())
  }

  fn handke_key_event(&mut self, event: KeyEvent) -> anyhow::Result<()> {
    match event.code {
      KeyCode::Char('q') => {
        if !self.state.editing {
          self.actions_tx.send(Action::Quit)?;
        }
      }
      KeyCode::Char('s') => {
        if self.state.currently_view != CurrentlyView::Search {
          self.state.currently_view = CurrentlyView::Search;

          self.actions_tx.send(Action::EnterSearchView)?;
        }
      }
      _ => {}
    };

    if let Some(action) = self.currently_view.on_key_event(event)? {
      self.actions_tx.send(action)?;
    }
    //
    //if let Some(action) = self.status_bar.on_key_event(event)? {
    //  self.actions_tx.send(action)?;
    //}

    Ok(())
  }

  fn render(&mut self, terminal: &mut Terminal) -> io::Result<()> {
    terminal.draw(|frame| {
      let bg = Block::default().bg(Color::Indexed(90));

      frame.render_widget(bg, frame.area());

      //self.status_bar.render(frame, layout_chunks[1]);
      self.currently_view.render(frame, frame.area());
    })?;

    Ok(())
  }
}

impl Default for App {
  fn default() -> Self {
    Self::new()
  }
}
