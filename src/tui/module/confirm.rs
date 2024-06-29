use ratatui::{
  layout::Rect,
  style::{Color, Style},
  widgets::{Block, Borders, Clear},
  Frame,
};

use crate::common::TecResult;

use super::{draw_input, Input};

pub struct Confirm {
  content: String,
  confirm: Input,
}

impl Default for Confirm {
  fn default() -> Self {
    Self {
      content: "".to_string(),
      confirm: Input::default().with_label("Yes/No: ").with_active(),
    }
  }
}

impl Confirm {
  pub fn with_content(mut self, content: impl Into<String>) -> Self {
    self.content = content.into();
    self
  }

  pub fn confirm(&mut self) -> bool {
    let res = self.confirm.content().eq_ignore_ascii_case("yes");
    self.confirm.reset();
    res
  }

  pub(crate) fn on_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> TecResult<()> {
    self.confirm.on_key_event(key_event)?;
    Ok(())
  }
}

pub fn draw_confirm(f: &mut Frame, confirm: &Confirm, area: Rect) {
  let block = Block::new()
    .title(confirm.content.as_str())
    .borders(Borders::ALL)
    .style(Style::default().bg(Color::Black).fg(Color::White));
  let inner_area = block.inner(area);

  f.render_widget(Clear, area);
  f.render_widget(block, area);
  draw_input(f, &confirm.confirm, inner_area);
}
