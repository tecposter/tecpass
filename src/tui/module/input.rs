use std::{cmp, usize};

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
  layout::Rect,
  style::{Modifier, Style, Stylize},
  text::{Line, Span},
  widgets::{Paragraph, Wrap},
  Frame,
};
use unicode_width::UnicodeWidthStr;

use crate::{common::TecResult, tui::util::get_pasted_content};

pub struct Input {
  label: String,
  content: String,
  pos: usize,
  is_active: bool,
  is_masked: bool,

  min: usize,
  max: usize,
  msg: String,
}

impl Default for Input {
  fn default() -> Self {
    Self {
      label: "".into(),
      content: "".into(),
      pos: 0,
      is_active: false,
      is_masked: false,
      min: 0,
      max: 200,
      msg: "".into(),
    }
  }
}

impl Input {
  pub fn with_label<T: Into<String>>(mut self, label: T) -> Self {
    self.label = label.into();
    self
  }

  pub fn with_mask(mut self) -> Self {
    self.is_masked = true;
    self
  }

  pub fn with_active(mut self) -> Self {
    self.activate();
    self
  }

  pub fn with_min(mut self, min: usize) -> Self {
    self.min = min;
    self
  }

  pub fn with_max(mut self, max: usize) -> Self {
    self.max = max;
    self
  }

  pub fn activate(&mut self) {
    self.is_active = true;
  }

  pub fn deactivate(&mut self) {
    self.is_active = false;
  }

  pub fn is_active(&self) -> bool {
    self.is_active
  }

  pub fn content(&self) -> &str {
    &self.content
  }

  // pub fn label(&self) -> &str {
  //   &self.label
  // }

  pub fn reset(&mut self) {
    self.content = "".into();
    self.pos = 0;
  }

  pub fn on_key_event(&mut self, key_event: KeyEvent) -> TecResult<()> {
    if !self.is_active {
      return Ok(());
    }

    let len = self.content.len();
    match key_event {
      KeyEvent {
        code: KeyCode::Char('v'),
        modifiers: KeyModifiers::CONTROL,
        kind: KeyEventKind::Press,
        ..
      } => {
        if len < self.max {
          if let Some(pasted) = get_pasted_content()? {
            self.insert_str(&pasted);
          }
        }
      }
      KeyEvent {
        kind: KeyEventKind::Press,
        ..
      } => match key_event.code {
        KeyCode::Char(ch) => {
          if len < self.max {
            self.enter_char(ch);
          }
        }
        KeyCode::Backspace => self.delete_char(),
        KeyCode::Left => self.move_cursor_left(),
        KeyCode::Right => self.move_cursor_right(),
        _ => {}
      },
      _ => {}
    }

    if !self.msg.is_empty() {
      self.msg = "".to_string();
    }

    Ok(())
  }

  pub fn validate(&mut self) -> bool {
    let len = self.content.len();
    if len >= self.min && len <= self.max {
      if !self.msg.is_empty() {
        self.msg = "".to_string();
      }
      true
    } else {
      self.msg = format!("The length is required be {}~{}", self.min, self.max,);
      false
    }
  }

  fn pos(&self) -> usize {
    self.pos
  }

  fn move_cursor_left(&mut self) {
    let left = self.pos.saturating_sub(1);
    self.pos = self.clamp_cursor(left);
  }

  fn move_cursor_right(&mut self) {
    let right = self.pos.saturating_add(1);
    self.pos = self.clamp_cursor(right);
  }

  fn enter_char(&mut self, ch: char) {
    let index = self.byte_idnex();
    self.content.insert(index, ch);
    self.move_cursor_right();
  }

  fn delete_char(&mut self) {
    let leftmost = self.pos == 0;
    if !leftmost {
      let curr = self.pos;
      let from = curr - 1;
      let before = self.content.chars().take(from);
      let after = self.content.chars().skip(curr);
      self.content = before.chain(after).collect();
      self.move_cursor_left();
    }
  }

  fn insert_str(&mut self, s: &str) {
    let curr = self.pos;
    let before = self.content.chars().take(curr);
    let after = self.content.chars().skip(curr);
    self.content = before.chain(s.chars()).chain(after).collect();
    let right = self.pos.saturating_add(s.len());
    self.pos = self.clamp_cursor(right);
  }

  fn byte_idnex(&self) -> usize {
    self
      .content
      .char_indices()
      .map(|(i, _)| i)
      .nth(self.pos)
      .unwrap_or(self.content.len())
  }

  fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
    new_cursor_pos.clamp(0, self.content.chars().count())
  }

  pub(crate) fn set_msg(&mut self, arg: impl Into<String>) {
    self.msg = arg.into();
  }

  pub(crate) fn toggle_mask(&mut self) {
    self.is_masked = !self.is_masked;
  }

  pub(crate) fn set_content(&mut self, content: impl Into<String>) {
    self.content = content.into();
  }

  pub fn width(&self) -> usize {
    self.content.width() + self.label.width()
  }
}

pub fn draw_input(f: &mut Frame, state: &Input, area: Rect) {
  let label = Span::styled(&state.label, Style::default().add_modifier(Modifier::BOLD));

  let content = {
    if state.is_masked {
      String::from_iter(state.content.chars().map(|_| '*'))
    } else {
      state.content.clone()
    }
  };

  let line_width = area.width as usize;
  let label_width = label.width();
  let content_width = content.width();

  let mut lines = vec![];
  let mut start = 0;
  let mut end = cmp::min(line_width - label_width, content_width);
  let line0 = Line::from(vec![label, Span::raw(&content[start..end])]);
  lines.push(line0);

  while end < content_width {
    start = end;
    end = cmp::min(start + line_width, content_width);
    lines.push(Line::from(&content[start..end]));
  }
  if !state.msg.is_empty() {
    let msg = format!(" {}", state.msg);
    lines
      .last_mut()
      .map(|l| l.push_span(Span::styled(msg, Style::new().red())));
  }

  let paragraph = Paragraph::new(lines).wrap(Wrap { trim: true });

  f.render_widget(paragraph, area);

  if state.is_active {
    let x = (label_width + state.pos()) % line_width;
    let y = (label_width + state.pos()) / line_width;
    #[allow(clippy::cast_possible_truncation)]
    f.set_cursor(area.x + (x as u16), area.y + (y as u16));
  }
}
