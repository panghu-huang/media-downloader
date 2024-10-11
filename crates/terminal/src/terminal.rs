use crossterm::event::{Event, EventStream, KeyEvent};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::CrosstermBackend;
use std::io::{self, Stdout};
use std::ops::{Deref, DerefMut};
use std::time::Duration;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tokio::time::interval;
use tokio_stream::StreamExt;

pub enum TerminalEvent {
  KeyEvent(KeyEvent),
  Tick,
  Quit,
}

pub struct Terminal {
  pub tick_rate: u8,
  pub tui: ratatui::Terminal<CrosstermBackend<Stdout>>,
  pub event_loop: Option<tokio::task::JoinHandle<()>>,
  pub sender: UnboundedSender<TerminalEvent>,
  pub receiver: UnboundedReceiver<TerminalEvent>,
}

impl Terminal {
  pub fn new(tick_rate: u8) -> io::Result<Self> {
    let (sender, receiver) = unbounded_channel();
    let backend = CrosstermBackend::new(io::stdout());
    let tui = ratatui::Terminal::new(backend)?;

    Ok(Terminal {
      tui,
      tick_rate,
      sender,
      receiver,
      event_loop: None,
    })
  }

  pub fn enter(&mut self) -> io::Result<()> {
    crossterm::execute!(io::stdout(), EnterAlternateScreen)?;
    crossterm::terminal::enable_raw_mode()?;

    self.event_loop();

    Ok(())
  }

  pub fn exit(&self) -> io::Result<()> {
    if let Some(event_loop) = &self.event_loop {
      event_loop.abort();
    }

    crossterm::execute!(io::stdout(), LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
  }

  pub async fn next_event(&mut self) -> Option<TerminalEvent> {
    self.receiver.recv().await
  }

  fn event_loop(&mut self) {
    let task = tokio::spawn({
      let sender = self.sender.clone();
      let tick_rate = self.tick_rate;
      //let frame_rate = self.frame_rate;

      async move {
        let mut tick_interval = interval(Duration::from_millis(1000 / tick_rate as u64));
        //let mut frame_interval = interval(Duration::from_millis(1000 / frame_rate as u64));
        let mut event_stream = EventStream::new();

        loop {
          let event = tokio::select! {
            _ = tick_interval.tick() => {
              TerminalEvent::Tick
            }
            //_ = frame_interval.tick() => {
            //  TerminalEvent::Frame
            //}
            event = event_stream.next() => {
              match event {
                Some(Ok(Event::Key(key_event))) => TerminalEvent::KeyEvent(key_event),
                _ => continue,
              }
            }
            Ok(_) = tokio::signal::ctrl_c() => {
              TerminalEvent::Quit
            }
          };

          if let Err(err) = sender.send(event) {
            eprintln!("Failed to send event: {}", err);
          }
        }
      }
    });

    self.event_loop = Some(task);
  }
}

impl Deref for Terminal {
  type Target = ratatui::Terminal<CrosstermBackend<Stdout>>;

  fn deref(&self) -> &Self::Target {
    &self.tui
  }
}

impl DerefMut for Terminal {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.tui
  }
}

impl Drop for Terminal {
  fn drop(&mut self) {
    self.exit().unwrap();
  }
}
