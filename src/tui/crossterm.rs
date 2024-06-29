use std::{
  io::{self, stdout},
  path::Path,
  time::{Duration, Instant},
};

use crossterm::{
  event::{self, DisableMouseCapture, EnableMouseCapture, Event},
  execute,
  terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
  backend::{Backend, CrosstermBackend},
  Terminal, TerminalOptions, Viewport,
};

use crate::common::TecResult;

use super::{
  app::{draw_app, App},
  auth::{draw_auth, Auth},
};

pub fn auth(tick_rate: Duration, config_path: impl AsRef<Path>) -> TecResult<Option<Vec<u8>>> {
  enable_raw_mode()?;
  let backend = CrosstermBackend::new(io::stdout());
  let mut terminal = Terminal::with_options(
    backend,
    TerminalOptions {
      viewport: Viewport::Inline(5),
    },
  )?;

  let auth = Auth::build(config_path)?;
  let res = run_auth(&mut terminal, auth, tick_rate);
  disable_raw_mode()?;
  res
}

fn run_auth<B: Backend>(
  terminal: &mut Terminal<B>,
  mut auth: Auth,
  tick_rate: Duration,
) -> TecResult<Option<Vec<u8>>> {
  let mut last_tick = Instant::now();
  loop {
    terminal.draw(|f| draw_auth(f, &mut auth))?;

    let timeout = tick_rate.saturating_sub(last_tick.elapsed());
    if event::poll(timeout)? {
      match event::read()? {
        Event::Key(key_event) => auth.on_key_event(key_event)?,
        _ => {}
      }
    }
    if last_tick.elapsed() >= tick_rate {
      last_tick = Instant::now();
    }

    let key = auth.key();
    if key.is_some() {
      return Ok(key);
    }

    if auth.quit() {
      return Ok(None);
    }
  }
}

pub fn run(tick_rate: Duration, config_path: impl AsRef<Path>, key: Vec<u8>) -> TecResult<()> {
  // setup terminal
  enable_raw_mode()?;
  let mut stdout = io::stdout();
  execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
  let backend = CrosstermBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;

  // create and run app
  let app = App::build(config_path, &key)?;
  let res = run_app(&mut terminal, app, tick_rate);

  // restore terminal
  disable_raw_mode()?;
  execute!(
    terminal.backend_mut(),
    LeaveAlternateScreen,
    DisableMouseCapture
  )?;
  terminal.show_cursor()?;

  // handle error
  if let Err(err) = res {
    println!("{err:?}");
  }

  Ok(())
}

fn run_app<B: Backend>(
  terminal: &mut Terminal<B>,
  mut app: App,
  tick_rate: Duration,
) -> TecResult<()> {
  let mut last_tick = Instant::now();
  loop {
    terminal.draw(|f| draw_app(f, &mut app))?;

    let timeout = tick_rate.saturating_sub(last_tick.elapsed());
    if event::poll(timeout)? {
      match event::read()? {
        Event::Key(key_event) => app.on_key_event(key_event)?,
        _ => {}
      }
    }
    if last_tick.elapsed() >= tick_rate {
      app.on_tick();
      last_tick = Instant::now();
    }
    if app.quit() {
      return Ok(());
    }
  }
  // Ok(())
}
