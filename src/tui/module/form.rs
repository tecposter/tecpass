use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
  layout::{Constraint, Layout, Rect},
  style::{Color, Style},
  widgets::{Block, Borders, Clear},
  Frame,
};

use crate::{
  common::TecResult,
  model::{Account, Pwd},
};

use super::{draw_input, Input};

#[derive(Clone, Copy)]
enum FormMode {
  Url = 0,
  Username,
  Password,
  Confirm,
}

impl FormMode {
  fn next(self) -> Self {
    let x = (self as i8 + 1) % 4;
    let item = unsafe { std::mem::transmute(x) };
    item
  }
  fn prev(self) -> Self {
    let x = (self as i8 - 1 + 4) % 4;
    let item = unsafe { std::mem::transmute(x) };
    item
  }
}

pub struct Form {
  mode: FormMode,
  url: Input,
  username: Input,
  password: Input,
  confirm: Input,
  // is_editing: bool,
}

impl Default for Form {
  fn default() -> Self {
    Self {
      mode: FormMode::Url,
      url: Input::default()
        .with_label("url: ")
        .with_min(1)
        .with_active(),
      username: Input::default().with_label("username: ").with_min(1),
      password: Input::default()
        .with_mask()
        .with_label("password: ")
        .with_min(8)
        .with_max(32),
      confirm: Input::default()
        .with_mask()
        .with_label("confirm: ")
        .with_min(8)
        .with_max(32),
      // is_editing: false,
    }
  }
}

impl Form {
  pub fn on_key_event(&mut self, key_event: KeyEvent) -> TecResult<()> {
    match key_event {
      KeyEvent {
        kind: KeyEventKind::Press,
        modifiers: KeyModifiers::CONTROL,
        code: KeyCode::Char('x'),
        ..
      } => {
        self.password.toggle_mask();
        self.confirm.toggle_mask();
      }
      KeyEvent {
        kind: KeyEventKind::Press,
        modifiers: KeyModifiers::CONTROL,
        code: KeyCode::Char('j'),
        ..
      }
      | KeyEvent {
        code: KeyCode::Down,
        kind: KeyEventKind::Press,
        ..
      }
      | KeyEvent {
        code: KeyCode::Tab,
        kind: KeyEventKind::Press,
        ..
      } => {
        self.next_mode();
      }
      KeyEvent {
        kind: KeyEventKind::Press,
        modifiers: KeyModifiers::CONTROL,
        code: KeyCode::Char('k'),
        ..
      }
      | KeyEvent {
        code: KeyCode::Up,
        kind: KeyEventKind::Press,
        ..
      } => {
        self.prev_mode();
      }
      _ => match self.mode {
        FormMode::Url => self.url.on_key_event(key_event)?,
        FormMode::Username => self.username.on_key_event(key_event)?,
        FormMode::Password => self.password.on_key_event(key_event)?,
        FormMode::Confirm => self.confirm.on_key_event(key_event)?,
      },
    }

    Ok(())
  }

  fn deactivate_all(&mut self) {
    self.url.deactivate();
    self.username.deactivate();
    self.password.deactivate();
    self.confirm.deactivate();
  }

  fn switch_inputs(&mut self) {
    self.deactivate_all();
    match self.mode {
      FormMode::Url => self.url.activate(),
      FormMode::Username => self.username.activate(),
      FormMode::Password => self.password.activate(),
      FormMode::Confirm => self.confirm.activate(),
    }
  }

  fn prev_mode(&mut self) {
    self.mode = self.mode.prev();
    self.switch_inputs();
  }
  fn next_mode(&mut self) {
    self.mode = self.mode.next();
    self.switch_inputs();
  }

  pub fn validate(&mut self) -> bool {
    if self.url.validate()
      && self.username.validate()
      && self.password.validate()
      && self.confirm.validate()
    {
      if self.password.content().eq(self.confirm.content()) {
        // if !self.msg.is_empty() {
        //   self.msg = "".to_owned();
        // }
        return true;
      } else {
        // if self.msg.is_empty() {
        //   self.msg = "Not match".to_owned();
        // }
        self.confirm.set_msg("Not match");
        return false;
      }
    }
    return false;
  }

  pub fn url(&self) -> &str {
    self.url.content()
  }

  pub fn username(&self) -> &str {
    self.username.content()
  }

  pub fn password(&self) -> &str {
    self.password.content()
  }

  pub(crate) fn reset(&mut self) {
    self.url.reset();
    self.username.reset();
    self.password.reset();
    self.confirm.reset();
  }

  pub(crate) fn load_account(&mut self, a: &Account, pwd: Option<&Pwd>) -> TecResult<()> {
    self.url.set_content(a.url());
    self.username.set_content(a.username());
    self.password.reset();
    self.confirm.reset();

    if let Some(p) = pwd {
      self.password.set_content(&p.password);
    }
    Ok(())
  }

  // pub(crate) fn is_editing(&self) -> bool {
  //   self.is_editing
  // }
}

pub fn draw_form(f: &mut Frame, form: &Form, area: Rect) {
  f.render_widget(Clear, area);

  // let title = {
  //   if form.is_editing {
  //     "Account (edit)"
  //   } else {
  //     "Account"
  //   }
  // };

  let block = Block::default()
    .title("Account")
    .borders(Borders::ALL)
    .style(Style::default().bg(Color::Black).fg(Color::White));
  let inner_area = block.inner(area);
  f.render_widget(block, area);

  let line_width = inner_area.width;
  let [url_area, username_area, password_area, confirm_area] = Layout::vertical([
    Constraint::Length(form.url.width().div_ceil(line_width as usize) as u16),
    Constraint::Length(form.username.width().div_ceil(line_width as usize) as u16),
    Constraint::Length(form.password.width().div_ceil(line_width as usize) as u16),
    Constraint::Length(form.confirm.width().div_ceil(line_width as usize) as u16),
  ])
  .areas(inner_area);

  draw_input(f, &form.url, url_area);
  draw_input(f, &form.username, username_area);
  draw_input(f, &form.password, password_area);
  draw_input(f, &form.confirm, confirm_area);
}
