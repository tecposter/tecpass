use std::{env, fs::create_dir_all, io::stdout, path::Path, thread::sleep, time::Duration};

use argh::FromArgs;
use common::TecResult;
use ratatui::{backend::CrosstermBackend, widgets::Paragraph, Terminal};
use tui::auth;

mod cipher;
mod common;
mod db;
mod hex;
mod import;
mod model;
mod repo;
mod tui;

/*
/// Demo
#[derive(Debug, FromArgs)]
struct Cli {
  /// config path, default ~/.config/tecpass
  #[argh(option, default = "~/.config/tecpass")]
  config_path: String,
  /// import firefox csv
  #[argh(option, default = "")]
  firefox_csv: String,
}
*/
/// TecPass
#[derive(Debug, FromArgs)]
struct Cli {
  /// time in ms between two ticks.
  #[argh(option, default = "200")]
  tick_rate: u64,
  /// config path, default ~/.config/tecpass
  #[argh(option, default = "String::from(\"~/.config/tecpass\")")]
  config_path: String,
  /// import firefox csv
  #[argh(option)]
  import_firefox_csv: Option<String>,
}

fn parse_config_path(path: &str) -> String {
  if path.starts_with('~') {
    if let Ok(home_dir) = env::var("HOME") {
      return home_dir + &path[1..];
    }
  }
  return path.to_string();
}

fn main() -> TecResult<()> {
  let cli: Cli = argh::from_env();
  let tick_rate = Duration::from_millis(cli.tick_rate);
  let config_path = parse_config_path(&cli.config_path);

  // let config_path = Path::new(&cli.config_path);
  create_dir_all(&config_path)?;

  let key = auth(tick_rate, &config_path)?;
  if key.is_none() {
    return Ok(());
  }

  if let Some(csv_path) = cli.import_firefox_csv {
    let db_path = Path::new(&config_path)
      .join("tecpass.db")
      .to_str()
      .unwrap()
      .to_owned();
    let key_bytes = key.unwrap();
    import::import_firefox_accounts(csv_path, db_path, &key_bytes)?;
    // import_firefox(import_firefox_csv)?;
    return Ok(());
  }
  // let tick_rate = Duration::from_millis(200);
  // let config_path = "./dev";

  tui::run(tick_rate, config_path, key.unwrap())?;
  Ok(())
}

fn import_firefox(csv_path: String) -> TecResult<()> {
  println!("{csv_path}");

  let backend = CrosstermBackend::new(stdout());
  let mut terminal = Terminal::with_options(
    backend,
    ratatui::TerminalOptions {
      viewport: ratatui::Viewport::Inline(3),
    },
  )?;
  terminal.draw(|frame| {
    let area = frame.size();
    frame.render_widget(Paragraph::new("Hello World!"), area);
    frame.set_cursor(area.x, area.y);
  })?;
  sleep(Duration::from_secs(10));
  Ok(())
}
