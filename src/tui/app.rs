use std::{path::Path, rc::Rc};

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
  layout::{Constraint, Direction, Layout, Rect},
  text::Line,
  Frame,
};

use crate::{
  cipher::AesCipher,
  common::TecResult,
  db::sqlite_conn,
  model::{Account, Pwd},
  repo::{AccountRepo, PwdRepo},
};

use super::{
  module::{
    draw_account_table, draw_confirm, draw_form, draw_view, AccountTable, Confirm, Form, View,
  },
  util::{copy_content, current_millis},
};

enum AppMode {
  Table,
  View,
  Add,
  Del,
  Edit,
}

pub struct App {
  mode: AppMode,
  account_repo: AccountRepo,
  pwd_repo: PwdRepo,

  quiting: bool,
  help_text: String,
  account_table: AccountTable,
  view: View,
  form: Form,
  to_del: Confirm,
}

impl App {
  pub fn build(config_path: impl AsRef<Path>, key: &[u8]) -> TecResult<Self> {
    let db_path = config_path.as_ref().join("tecpass.db");
    let conn = Rc::new(sqlite_conn(db_path)?);
    let cipher = Rc::new(AesCipher::from_slice(key)?);
    let account_repo = AccountRepo::new(conn.clone(), cipher.clone());
    let pwd_repo = PwdRepo::new(conn, cipher);

    let mut app = Self {
      // account_repo: AccountRepo::new(conn),
      mode: AppMode::Table,
      account_repo,
      pwd_repo,
      quiting: false,
      help_text: "".to_owned(),
      account_table: AccountTable::default(),
      view: View::default(),
      form: Form::default(),
      to_del: Confirm::default().with_content("To delete the selected account?"),
    };

    app.change_mode(AppMode::Table);
    app.load_accounts()?;

    // app.change_mode(mode);
    Ok(app)
  }

  pub fn on_key_event(&mut self, key_event: KeyEvent) -> TecResult<()> {
    // Press `Ctrl-c` to quit
    if key_event.kind == KeyEventKind::Press
      && key_event.modifiers == KeyModifiers::CONTROL
      && key_event.code == KeyCode::Char('c')
    {
      self.quiting = true;
      return Ok(());
    }
    match self.mode {
      AppMode::Table => self.table_on_key_envent(key_event)?,
      AppMode::View => self.view_on_key_event(key_event)?,
      AppMode::Add => self.add_on_key_event(key_event)?,
      AppMode::Del => self.del_on_key_event(key_event)?,
      AppMode::Edit => self.edit_on_key_event(key_event)?,
    }

    Ok(())
  }

  pub(crate) fn on_tick(&self) {}

  pub(crate) fn quit(&self) -> bool {
    self.quiting
  }

  fn table_on_key_envent(&mut self, key_event: KeyEvent) -> TecResult<()> {
    if !self.account_table.is_querying() {
      match key_event {
        KeyEvent {
          code,
          kind: KeyEventKind::Press,
          ..
        } => match code {
          KeyCode::Enter | KeyCode::Char('l') => {
            if let Some(account) = self.account_table.selected() {
              self.view.load_account(account.clone());
              let pwds = self.pwd_repo.query(account.id)?;
              self.view.load_pwds(pwds);
              self.change_mode(AppMode::View);
            }
          }
          KeyCode::Char('a') => {
            self.form.reset();
            self.change_mode(AppMode::Add);
          }
          KeyCode::Char('e') => {
            if let Some(acc) = self.account_table.selected() {
              let pwds = self.pwd_repo.query(acc.id)?;
              self.form.load_account(acc, pwds.get(0))?;
              self.change_mode(AppMode::Edit);
            }
          }
          KeyCode::Char('d') => {
            self.change_mode(AppMode::Del);
          }
          KeyCode::Char('c') => {
            self.copy()?;
          }
          _ => {}
        },
        _ => {}
      }
    }
    self.account_table.on_key_event(key_event)?;
    Ok(())
  }

  fn view_on_key_event(&mut self, key_event: KeyEvent) -> TecResult<()> {
    match key_event {
      KeyEvent {
        code: KeyCode::Esc | KeyCode::Char('q'),
        kind: KeyEventKind::Press,
        ..
      } => {
        self.change_mode(AppMode::Table);
      }
      _ => {
        self.view.on_key_event(key_event)?;
      }
    }
    Ok(())
  }
  fn add_on_key_event(&mut self, key_event: KeyEvent) -> TecResult<()> {
    match key_event {
      KeyEvent {
        code: KeyCode::Esc,
        kind: KeyEventKind::Press,
        ..
      } => {
        self.change_mode(AppMode::Table);
      }
      KeyEvent {
        code: KeyCode::Enter,
        kind: KeyEventKind::Press,
        ..
      } => {
        if self.form.validate() {
          let current = current_millis() as usize;
          let acc = Account {
            id: 0,
            url: self.form.url().to_string(),
            username: self.form.username().to_string(),
            created: current,
            changed: current,
          };
          let aid = self.account_repo.add(&acc)?;
          let pwd = Pwd {
            id: 0,
            aid,
            password: self.form.password().to_string(),
            created: current,
          };
          self.pwd_repo.add(&pwd)?;

          self.load_accounts()?;
          self.account_table.select_by_aid(aid);

          self.form.reset();
          self.change_mode(AppMode::Table);
        }
      }
      _ => self.form.on_key_event(key_event)?,
    }
    Ok(())
  }

  fn edit_on_key_event(&mut self, key_event: KeyEvent) -> TecResult<()> {
    match key_event {
      KeyEvent {
        code: KeyCode::Esc,
        kind: KeyEventKind::Press,
        ..
      } => {
        self.change_mode(AppMode::Table);
        return Ok(());
      }
      KeyEvent {
        code: KeyCode::Enter,
        kind: KeyEventKind::Press,
        ..
      } => {
        if self.form.validate() {
          let current = current_millis() as usize;
          if let Some(selected) = self.account_table.selected() {
            let aid = selected.id;
            let acc = Account {
              id: aid,
              url: self.form.url().to_string(),
              username: self.form.username().to_string(),
              created: current,
              changed: current,
            };
            self.account_repo.update(&acc)?;
            let pwd = Pwd {
              id: 0,
              aid: selected.id,
              password: self.form.password().to_string(),
              created: current,
            };
            self.pwd_repo.add(&pwd)?;

            // self.account_table.load(self.account_repo.all()?);
            self.load_accounts()?;
            self.account_table.select_by_aid(aid);

            self.form.reset();
            self.change_mode(AppMode::Table);
          }
          return Ok(());
        }
      }
      _ => self.form.on_key_event(key_event)?,
    }
    Ok(())
  }

  fn del_on_key_event(&mut self, key_event: KeyEvent) -> TecResult<()> {
    match key_event {
      KeyEvent {
        code: KeyCode::Esc,
        kind: KeyEventKind::Press,
        ..
      } => self.change_mode(AppMode::Table),
      KeyEvent {
        code: KeyCode::Enter,
        kind: KeyEventKind::Press,
        ..
      } => {
        if self.to_del.confirm() {
          if let Some(acc) = self.account_table.selected() {
            self.account_repo.delete(acc.id)?;
            self.pwd_repo.delete(acc.id)?;
            self.account_table.load(self.account_repo.all()?);
          }
        }
        self.change_mode(AppMode::Table);
      }
      _ => self.to_del.on_key_event(key_event)?,
    }
    Ok(())
  }

  fn change_mode(&mut self, mode: AppMode) {
    self.mode = mode;
    match self.mode {
      AppMode::Table => {
        self.help_text =
          "/: filter, a: add, e: edit, d: delete, c: copy password, j: next, k: prev, l/enter: view, ctrl-c: quit"
            .to_owned()
      }
      AppMode::View => {
        self.help_text = "View Account - c: copy, j: next, k: prev, x: show/hide passwords, q/esc: back".to_owned()
      }
      AppMode::Add => {
        self.help_text =
          "Eidt Account - down/ctrl-j: next, up/ctrl-k: prev, ctrl-x: show/hide passwords, ctrl-v: paste, esc: back".to_owned()
      }
      AppMode::Del => self.help_text = "Delete Account - esc: back".to_owned(),
      AppMode::Edit => {
        self.help_text =
          "Edit Account - ctrl-j: next, ctrl-k: prev, ctrl-x: show/hide passwords, ctrl-v: paste, esc: back".to_owned()
      }
    }
  }

  fn copy(&mut self) -> TecResult<()> {
    if let Some(account) = self.account_table.selected() {
      let pwds = self.pwd_repo.query(account.id)?;
      if let Some(pwd) = pwds.get(0) {
        copy_content(pwd.password.as_bytes())?;
      }
    }
    Ok(())
  }

  fn load_accounts(&mut self) -> TecResult<()> {
    let accounts = self.account_repo.all()?;
    self.account_table.load(accounts);
    Ok(())
  }
}

pub fn draw_app(f: &mut Frame, app: &mut App) {
  let [main_area, help_area] = Layout::vertical([
    Constraint::Min(3),
    // Constraint::Length(1),
    Constraint::Length(1),
  ])
  .areas(f.size());

  draw_account_table(f, &mut app.account_table, main_area);

  let pop_rect = centered_rect(60, 30, main_area);
  match app.mode {
    // AppMode::Table => {
    //   draw_account_table(f, &mut app.account_table, main_area, search_area);
    // }
    AppMode::View => {
      // f.render_widget(Line::raw("test"), centered_rect(60, 25, main_area));
      // draw_view(f, &mut app.view, main_area);
      draw_view(f, &mut app.view, pop_rect);
    }
    AppMode::Add => draw_form(f, &app.form, pop_rect),
    AppMode::Edit => draw_form(f, &app.form, pop_rect),
    AppMode::Del => draw_confirm(f, &app.to_del, centered_rect(60, 6, main_area)),
    AppMode::Table => {}
  }
  // match app.mode {
  //   AppMode::Table => {
  //     draw_account_table(f, &mut app.account_table, main_area, search_area);
  //   } // _ => {}
  // }
  f.render_widget(Line::raw(&app.help_text), help_area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
  // Cut the given rectangle into three vertical pieces
  let popup_layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
      Constraint::Percentage((100 - percent_y) / 2),
      Constraint::Percentage(percent_y),
      Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(r);

  // Then cut the middle vertical piece into three width-wise pieces
  Layout::default()
    .direction(Direction::Horizontal)
    .constraints([
      Constraint::Percentage((100 - percent_x) / 2),
      Constraint::Percentage(percent_x),
      Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(popup_layout[1])[1] // Return the middle chunk
}

// fn current_timestamp() -> usize {
//   let now = SystemTime::now();
//   let since_the_epoch = now
//     .duration_since(UNIX_EPOCH)
//     .expect("Clock may have gone backwards");
//   since_the_epoch.as_millis() as usize
// }
