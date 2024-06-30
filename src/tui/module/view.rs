use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
  layout::Rect,
  style::{Color, Modifier, Style, Stylize},
  text::{Line, Span},
  widgets::{Block, Borders, Clear, List, ListItem, ListState},
  Frame,
};

use crate::{
  common::TecResult,
  model::{Account, Pwd},
  tui::util::{copy_content, millis2string},
};

pub struct View {
  account: Option<Account>,
  pwds: Option<Vec<Pwd>>,
  is_masked: bool,
  state: ListState,
  symbol: String,
}

impl Default for View {
  fn default() -> Self {
    Self {
      account: None,
      pwds: None,
      is_masked: true,
      state: ListState::default().with_selected(Some(0)),
      symbol: "❯".into(),
    }
  }
}

impl View {
  pub fn load_account(&mut self, account: Account) {
    self.account = Some(account);
    self.state.select(Some(0));
  }

  pub fn load_pwds(&mut self, pwds: Vec<Pwd>) {
    self.pwds = Some(pwds);
  }

  // pub fn show_pwds(&mut self) {
  //   self.is_masked = false;
  // }
  // pub fn hide_pwds(&mut self) {
  //   self.is_masked = true;
  // }

  pub(crate) fn on_key_event(&mut self, key_event: KeyEvent) -> TecResult<()> {
    self.symbol = "❯".into();
    match key_event {
      KeyEvent {
        code,
        kind: KeyEventKind::Press,
        ..
      } => match code {
        KeyCode::Down | KeyCode::Char('j') => self.next(),
        KeyCode::Up | KeyCode::Char('k') => self.prev(),
        // KeyCode::Enter | KeyCode::Char('l') => self.select(),
        KeyCode::Char('c') => self.copy()?,
        KeyCode::Char('x') => self.is_masked = !self.is_masked,
        _ => {}
      },
      _ => {}
    }
    Ok(())
  }

  fn next(&mut self) {
    let len = 5;
    let select = (self.state.selected().unwrap_or(0) + 1) % len;
    self.state.select(Some(select));
  }

  fn prev(&mut self) {
    let len = 5;
    let select = (self.state.selected().unwrap_or(0) + len - 1) % len;
    self.state.select(Some(select));
  }

  fn copy(&mut self) -> TecResult<()> {
    if self.account.is_none() {
      return Ok(());
    }
    let account = self.account.as_ref().unwrap();
    match self.state.selected() {
      Some(0) => copy_content(account.url.as_bytes())?,
      Some(1) => copy_content(account.username.as_bytes())?,
      Some(2) => {
        if let Some(pwds) = self.pwds.as_ref() {
          if let Some(pwd) = pwds.get(0) {
            copy_content(pwd.password.as_bytes())?;
          }
        }
      }
      Some(3) => copy_content(millis2string(account.created as u64).as_bytes())?,
      Some(4) => copy_content(millis2string(account.changed as u64).as_bytes())?,
      _ => {}
    }
    if self.state.selected().is_some() {
      self.symbol = "✔".into();
    }
    Ok(())
  }
}

pub fn draw_view(f: &mut Frame, view: &mut View, area: Rect) {
  // let url_label = Span::styled("url: ", Style::default().bold());

  if view.account.is_none() {
    return;
  }

  // f.render_widget(
  //   Line::raw("=====xxxxxxxxxxxxxxxxxxx======================="),
  //   area,
  // );
  // return;

  let account = view.account.as_ref().unwrap();
  let url_item: ListItem = Line::from(vec![
    Span::styled("url: ", Style::default().bold()),
    Span::raw(&account.url),
  ])
  .into();
  let username_item: ListItem = Line::from(vec![
    Span::styled("username: ", Style::default().bold()),
    Span::raw(&account.username),
  ])
  .into();

  let created_item: ListItem = Line::from(vec![
    Span::styled("created: ", Style::default().bold()),
    Span::raw(millis2string(account.created as u64)),
  ])
  .into();
  let changed_item: ListItem = Line::from(vec![
    Span::styled("changed: ", Style::default().bold()),
    Span::raw(millis2string(account.changed as u64)),
  ])
  .into();

  let mut pwds_line = Line::from(vec![Span::styled("passwords: ", Style::default().bold())]);
  if view.is_masked {
    pwds_line.push_span(Span::raw("*******"))
  } else if let Some(pwds) = view.pwds.as_mut() {
    let pwds_str = pwds
      .iter()
      .map(|p| p.password.clone())
      .collect::<Vec<String>>()
      .join("; ");
    pwds_line.push_span(Span::raw(pwds_str));
    // pwds.iter().map(|p| p.password)
  }
  let pwds_item: ListItem = pwds_line.into();

  let block = Block::default()
    .title("Account")
    .borders(Borders::ALL)
    .style(Style::default().bg(Color::LightYellow).fg(Color::Black));

  let list = List::new(vec![
    url_item,
    username_item,
    pwds_item,
    created_item,
    changed_item,
  ])
  .block(block)
  .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
  .highlight_symbol(view.symbol.as_str());
  // let paragraph = Paragraph::new(vec![
  //   url_line,
  //   username_line,
  //   pwds_line,
  //   created_line,
  //   changed_line,
  // ])
  // .block(block)
  // .wrap(Wrap { trim: true });

  f.render_widget(Clear, area);
  // f.render_widget(paragraph, area);
  if view.state.selected().is_none() {
    view.state.select(Some(0));
  }
  f.render_stateful_widget(list, area, &mut view.state);
}
