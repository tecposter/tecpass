use std::u16;

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
  layout::{Constraint, Layout, Rect},
  widgets::{Block, Borders},
  Frame,
};

use crate::common::TecResult;

use super::{draw_input, Input};

pub struct ConfirmPassword {
  title: String,
  password: Input,
  confirm: Input,
  // msg: String,
}

impl Default for ConfirmPassword {
  fn default() -> Self {
    Self {
      title: "".to_string(),
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
    }
  }
}

impl ConfirmPassword {
  pub fn with_title(mut self, title: impl Into<String>) -> Self {
    self.title = title.into();
    self
  }

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
    // if !self.msg.is_empty() {
    //   self.msg = "".to_owned();
    // }
    Ok(())
  }

  pub fn validate(&mut self) -> bool {
    if !self.password.validate() || !self.confirm.validate() {
      return false;
    }
    if self.password.content().eq(self.confirm.content()) {
      // if !self.msg.is_empty() {
      //   self.msg = "".to_owned();
      // }
      return true;
    } else {
      self.confirm.set_msg("Not match");
      // if self.msg.is_empty() {
      //   self.msg = "Not match".to_owned();
      // }
      return false;
    }
  }

  pub fn content(&self) -> &str {
    self.password.content()
  }
}

pub fn draw_confirm_password(f: &mut Frame, state: &ConfirmPassword, area: Rect) {
  let block = Block::new()
    .title(state.title.as_str())
    .borders(Borders::ALL);
  let inner_area = block.inner(area);
  f.render_widget(block, area);

  let pwd_height = state.password.width().div_ceil(area.width as usize) as u16;
  let confirm_height = state.confirm.width().div_ceil(area.width as usize) as u16;
  let [pwd_area, confirm_area] = Layout::vertical([
    Constraint::Length(pwd_height),
    Constraint::Length(confirm_height),
  ])
  .areas(inner_area);
  draw_input(f, &state.password, pwd_area);
  draw_input(f, &state.confirm, confirm_area);
}
