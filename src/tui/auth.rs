use std::path::{Path, PathBuf};

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::Frame;

use crate::{common::TecResult, db::KeyStore};

use super::module::{draw_confirm_password, draw_input, ConfirmPassword, Input};

enum AuthMode {
  Reg,
  Login,
}

pub struct Auth {
  mode: AuthMode,
  key_store: KeyStore<PathBuf>,

  quiting: bool,
  key: Option<Vec<u8>>,

  login: Input,
  reg: ConfirmPassword,
}

impl Auth {
  pub fn build(config_path: impl AsRef<Path>) -> TecResult<Self> {
    let key_path = config_path.as_ref().join("tecpass.key");

    let key_store = KeyStore::new(key_path);
    let mode = {
      if key_store.has_key() {
        AuthMode::Login
      } else {
        AuthMode::Reg
      }
    };
    let mut auth = Self {
      // account_repo: AccountRepo::new(conn),
      key_store,
      mode: AuthMode::Login,
      quiting: false,
      key: None,
      login: Input::default()
        .with_mask()
        .with_label("password: ")
        .with_min(8)
        .with_max(32)
        .with_active(),
      reg: ConfirmPassword::default(),
    };
    auth.change_mode(mode);
    Ok(auth)
  }

  pub fn quit(&self) -> bool {
    self.quiting
  }

  pub fn key(&mut self) -> Option<Vec<u8>> {
    self.key.take()
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
      AuthMode::Login => self.login_on_key_event(key_event)?,
      AuthMode::Reg => self.reg_on_key_envent(key_event)?,
    }

    Ok(())
  }

  fn login_on_key_event(&mut self, key_event: KeyEvent) -> TecResult<()> {
    self.login.on_key_event(key_event)?;
    if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::Enter {
      if self.login.validate() {
        let pwd = self.login.content();
        let res = self.key_store.get_key(pwd.as_bytes());
        if let Ok(key) = res {
          self.key = Some(key);
          // self.change_mode(AppMode::Table);
        } else {
          self.login.set_msg("wrong password");
        }
      }
    }
    Ok(())
  }

  fn reg_on_key_envent(&mut self, key_event: KeyEvent) -> TecResult<()> {
    self.reg.on_key_event(key_event)?;
    if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::Enter {
      if self.reg.validate() {
        let pwd = self.reg.content();
        self.key_store.set_key(pwd.as_bytes())?;
        self.change_mode(AuthMode::Login);
      }
    }
    Ok(())
  }

  fn change_mode(&mut self, mode: AuthMode) {
    self.mode = mode
  }
}
pub fn draw_auth(f: &mut Frame, auth: &mut Auth) {
  let area = f.size();

  match auth.mode {
    AuthMode::Login => {
      draw_input(f, &auth.login, area);
    }
    AuthMode::Reg => draw_confirm_password(f, &auth.reg, area),
  }
}
