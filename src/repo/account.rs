use std::rc::Rc;

use rusqlite::Connection;

use crate::{cipher::AesCipher, common::TecResult, model::Account};

pub struct AccountRepo {
  // conn: &'Connection,
  conn: Rc<Connection>,
  cipher: Rc<AesCipher>,
}

/*
CREATE TABLE if not exists account (
    id    INTEGER PRIMARY KEY,
    url  TEXT NOT NULL,
    username TEXT NOT NULL,
    created INTEGER,
    changed INTEGER
);
*/

impl AccountRepo {
  pub fn new(conn: Rc<Connection>, cipher: Rc<AesCipher>) -> Self {
    Self { conn, cipher }
  }

  // to read
  // Is there a way to "Get or Insert" in a single query_row call? #1349
  // https://github.com/rusqlite/rusqlite/discussions/1349
  pub fn add(&self, a: &Account) -> TecResult<u32> {
    let url = Some(self.cipher.encypt(a.url().as_bytes())?);
    let username = Some(self.cipher.encypt(a.username().as_bytes())?);
    let id = self.conn.query_row(
      "INSERT INTO account (url, username, created, changed) VALUES (?1, ?2, ?3, ?4) RETURNING id",
      (&url, &username, &a.created, &a.changed),
      |row| row.get(0),
    )?;
    // self.conn.execute(
    //   "INSERT INTO account (url, username, created, changed) VALUES (?1, ?2, ?3, ?4)",
    //   (&a.url, &a.username, &a.created, &a.changed),
    // )?;
    Ok(id)
  }

  pub(crate) fn update(&self, a: &Account) -> TecResult<()> {
    let url = Some(self.cipher.encypt(a.url().as_bytes())?);
    let username = Some(self.cipher.encypt(a.username().as_bytes())?);
    let mut stmt = self
      .conn
      .prepare("UPDATE account SET url = ?1, username = ?2, changed = ?3 WHERE id=?4")?;
    stmt.execute((&url, &username, &a.changed, &a.id))?;
    Ok(())
  }

  pub(crate) fn delete(&self, id: u32) -> TecResult<()> {
    let mut stmt = self.conn.prepare("DELETE FROM account WHERE id = ?1")?;
    stmt.execute([id])?;
    Ok(())
  }

  // todo
  // Iterator
  // https://github.com/rusqlite/rusqlite/discussions/1198
  pub fn all(&self) -> TecResult<Vec<Account>> {
    let mut stmt = self
      .conn
      .prepare("SELECT id, url, username, created, changed FROM account")?;
    let iter = stmt.query_map([], |row| {
      let cipher_url: Vec<u8> = row.get(1)?;
      let cipher_username: Vec<u8> = row.get(2)?;
      let plain_url = self.cipher.decypt(&cipher_url).unwrap();
      let plain_username = self.cipher.decypt(&cipher_username).unwrap();
      Ok(Account {
        id: row.get(0)?,
        // url: row.get(1)?,
        // username: row.get(2)?,
        url: String::from_utf8(plain_url).unwrap(),
        username: String::from_utf8(plain_username).unwrap(),
        created: row.get(3)?,
        changed: row.get(4)?,
      })
    })?;

    Ok(iter.collect::<rusqlite::Result<Vec<Account>>>()?)
  }

  // pub fn query(&self, q: &str) -> TecResult<Vec<Account>> {
  //   let mut stmt = self.conn.prepare(
  //     "SELECT id, url, username, created, changed FROM account where url LIKE ?1 or username LIKE ?1",
  //   )?;
  //   let query = format!("%{q}%");
  //   let iter = stmt.query_map(rusqlite::params![&query], |row| {
  //     Ok(Account {
  //       id: row.get(0)?,
  //       url: row.get(1)?,
  //       username: row.get(2)?,
  //       created: row.get(3)?,
  //       changed: row.get(4)?,
  //     })
  //   })?;

  //   Ok(iter.collect::<rusqlite::Result<Vec<Account>>>()?)
  // }
}
