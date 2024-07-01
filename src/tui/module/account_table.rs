use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
  layout::{Constraint, Layout, Rect},
  style::{Color, Style, Stylize},
  widgets::{Block, Row, Table, TableState},
  Frame,
};

use crate::{common::TecResult, model::Account};

use super::{draw_input, Input};

pub struct AccountIter<'a> {
  accounts: &'a [Account],
  filtered: &'a [usize],
  curr: usize,
}

impl<'a> Iterator for AccountIter<'a> {
  type Item = &'a Account;

  fn next(&mut self) -> Option<Self::Item> {
    let curr = self.curr;
    self.curr += 1;
    if let Some(&pos) = self.filtered.get(curr) {
      self.accounts.get(pos)
    } else {
      None
    }
  }
}

pub struct AccountVec {
  accounts: Vec<Account>,
  filtered: Vec<usize>,
}

impl AccountVec {
  pub fn new(accounts: Vec<Account>) -> Self {
    let len = accounts.len();
    Self {
      accounts,
      filtered: (0..len).collect(),
    }
  }

  pub fn load(&mut self, accounts: Vec<Account>) {
    let filtered = (0..accounts.len()).collect();
    self.accounts = accounts;
    self.filtered = filtered;
  }

  pub fn filter(&mut self, query: &str) {
    if !query.is_empty() {
      let mut res = vec![];
      for (i, a) in self.accounts.iter().enumerate() {
        if a.url.contains(query) || a.username.contains(query) {
          res.push(i);
        }
      }
      self.filtered = res;
    } else {
      self.filtered = (0..self.accounts.len()).collect();
    }
  }

  fn get(&self, index: usize) -> Option<&Account> {
    if let Some(&pos) = self.filtered.get(index) {
      return self.accounts.get(pos);
    }
    None
  }

  fn index(&self, aid: u32) -> Option<usize> {
    if let Some(pos) = self.accounts.iter().position(|a| a.id == aid) {
      return self.filtered.iter().position(|p| p.eq(&pos));
    }
    None
  }

  pub fn iter(&self) -> AccountIter {
    AccountIter {
      accounts: &self.accounts,
      filtered: &self.filtered,
      curr: 0,
    }
  }
}

pub struct AccountTable {
  items: AccountVec,
  query: Input,
  state: TableState,
  query_content: String,
  symbol: String,
}

impl Default for AccountTable {
  fn default() -> Self {
    Self {
      items: AccountVec::new(vec![]),
      query: Input::default().with_label("/".to_owned()),
      state: TableState::default().with_selected(Some(0)),
      query_content: "".to_owned(),
      symbol: "❯".into(),
    }
  }
}

impl AccountTable {
  pub fn on_key_event(&mut self, key_event: KeyEvent) -> TecResult<()> {
    if self.query.is_active() {
      if key_event.kind == KeyEventKind::Press
        && (key_event.code == KeyCode::Esc || key_event.code == KeyCode::Enter)
      {
        self.query.deactivate();
        return Ok(());
      }
      self.query.on_key_event(key_event)?;
      if !self.query_content.eq(self.query.content()) {
        self.items.filter(self.query.content());
        self.state.select(Some(0));
        self.query_content = self.query.content().to_string();
      }
    } else {
      match key_event {
        KeyEvent {
          code,
          kind: KeyEventKind::Press,
          ..
        } => match code {
          KeyCode::Down | KeyCode::Char('j') => self.next(),
          KeyCode::Up | KeyCode::Char('k') => self.prev(),
          KeyCode::Char('/') => self.query.activate(),
          _ => {}
        },
        _ => {}
      }
    }
    self.symbol = "❯".into();
    Ok(())
  }

  pub fn load(&mut self, accounts: Vec<Account>) {
    self.items.load(accounts);
    self.query.reset();
  }

  pub fn is_querying(&self) -> bool {
    self.query.is_active()
  }

  fn next(&mut self) {
    let len = self.items.filtered.len();
    if len > 0 {
      let select = (self.state.selected().unwrap_or(0) + 1) % len;
      self.state.select(Some(select));
    }
  }

  fn prev(&mut self) {
    let len = self.items.filtered.len();
    if len > 0 {
      let select = (self.state.selected().unwrap_or(0) + len - 1) % len;
      self.state.select(Some(select));
    }
  }

  pub(crate) fn selected(&mut self) -> Option<&Account> {
    if let Some(index) = self.state.selected() {
      self.symbol = "✔".into();
      self.items.get(index)
    } else {
      None
    }
  }

  pub(crate) fn select_by_aid(&mut self, aid: u32) {
    self.state.select(self.items.index(aid))
  }
}

pub fn draw_account_table(f: &mut Frame, at: &mut AccountTable, area: Rect) {
  let [main_area, search_area] =
    Layout::vertical([Constraint::Min(3), Constraint::Length(1)]).areas(area);
  let rows: Vec<Row> = at
    .items
    .iter()
    .map(|a| Row::new(vec![a.url(), a.username()]))
    .collect();
  let widths = [Constraint::Min(10), Constraint::Min(5)];
  let table = Table::new(rows, widths)
    .column_spacing(1)
    .style(Style::new().blue())
    .header(
      Row::new(vec!["url", "username"]).style(Style::new().bg(Color::LightYellow).fg(Color::Black)),
    )
    .style(Style::new().bold())
    .block(Block::default())
    .highlight_style(Style::new().reversed())
    .highlight_symbol(at.symbol.as_str());

  f.render_stateful_widget(table, main_area, &mut at.state);

  if at.query.is_active() || !at.query.content().is_empty() {
    draw_input(f, &at.query, search_area);
  }
}
