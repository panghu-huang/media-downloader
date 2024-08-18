use crate::terminal::{Terminal, TerminalEvent};
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use std::io;

#[derive(Default)]
pub struct App {
  should_exit: bool,
}

impl App {
  pub async fn run(&mut self) -> anyhow::Result<()> {
    let mut terminal = Terminal::new(4, 60)?;

    terminal.enter()?;

    loop {
      self.handle_event(&mut terminal).await?;

      if self.should_exit {
        break;
      }

      self.render(&mut terminal)?;
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
        self.handke_key_event(key_event);
      }
      _ => {}
    }

    Ok(())
  }

  fn handke_key_event(&mut self, event: KeyEvent) {
    match event.code {
      KeyCode::Char('q') => {
        self.should_exit = true;
      }
      KeyCode::Tab => {
        // Do something
      }
      _ => {}
    }
  }

  fn render(&mut self, terminal: &mut Terminal) -> io::Result<()> {
    terminal.draw(|frame| {
      let text = ratatui::text::Text::from("Hello, World!");

      frame.render_widget(text, frame.area());
    })?;

    Ok(())
  }
}
