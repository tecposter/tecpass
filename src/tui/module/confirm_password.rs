use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
  layout::{Constraint, Layout, Rect},
  style::{Style, Stylize},
  text::Line,
  Frame,
};

use crate::common::TecResult;

use super::{draw_input, Input};

pub struct ConfirmPassword {
  password: Input,
  confirm: Input,
  msg: String,
}

impl Default for ConfirmPassword {
  fn default() -> Self {
    Self {
      password: Input::default()
        .with_mask()
        .with_label("password: ")
        .with_min(8)
        .with_max(32)
        .with_active(),
      confirm: Input::default()
        .with_mask()
        .with_label("confirm: ")
        .with_min(8)
        .with_max(32),
      msg: "".to_owned(),
    }
  }
}

impl ConfirmPassword {
  pub fn on_key_event(&mut self, key_event: KeyEvent) -> TecResult<()> {
    // press `Tab` to switch input
    if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::Tab {
      if self.password.is_active() {
        self.password.deactivate();
        self.confirm.activate();
      } else {
        self.password.activate();
        self.confirm.deactivate();
      }
    }
    let curr = {
      if self.password.is_active() {
        &mut self.password
      } else {
        &mut self.confirm
      }
    };
    curr.on_key_event(key_event)?;
    if !self.msg.is_empty() {
      self.msg = "".to_owned();
    }
    Ok(())
  }

  pub fn validate(&mut self) -> bool {
    if !self.password.validate() || !self.confirm.validate() {
      return false;
    }
    if self.password.content().eq(self.confirm.content()) {
      if !self.msg.is_empty() {
        self.msg = "".to_owned();
      }
      return true;
    } else {
      if self.msg.is_empty() {
        self.msg = "Not match".to_owned();
      }
      return false;
    }
  }

  pub fn content(&self) -> &str {
    self.password.content()
  }
}

pub fn draw_confirm_password(f: &mut Frame, state: &ConfirmPassword, area: Rect) {
  let [pwd_area, confirm_area, msg_area] = Layout::vertical([
    Constraint::Max(2),
    Constraint::Max(2),
    Constraint::Length(1),
  ])
  .areas(area);
  draw_input(f, &state.password, pwd_area);
  draw_input(f, &state.confirm, confirm_area);
  f.render_widget(Line::styled(&state.msg[..], Style::new().red()), msg_area);
}
