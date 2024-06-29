use std::rc::Rc;

use rusqlite::Connection;

use crate::{cipher::AesCipher, common::TecResult, model::Pwd};

/*
CREATE TABLE if not exists pwd (
    id INTEGER PRIMARY KEY,
    aid INTEGER NOT NULL,
    password TEXT NOT NULL,
    created INTEGER
);
*/

pub struct PwdRepo {
  // conn: &'a Connection,
  conn: Rc<Connection>,
  cipher: Rc<AesCipher>,
}

impl PwdRepo {
  pub fn new(conn: Rc<Connection>, cipher: Rc<AesCipher>) -> Self {
    Self { conn, cipher }
  }

  pub fn add(&self, pwd: &Pwd) -> TecResult<u32> {
    let password = Some(self.cipher.encypt(pwd.password.as_bytes())?);
    let id: u32 = self.conn.query_row(
      "INSERT INTO pwd (aid, password, created) VALUES (?1, ?2, ?3) RETURNING id",
      (&pwd.aid, &password, &pwd.created),
      |row| row.get(0),
    )?;
    Ok(id)
  }

  pub fn query(&self, aid: u32) -> TecResult<Vec<Pwd>> {
    let mut stmt = self
      .conn
      .prepare("SELECT id, aid, password, created FROM pwd WHERE aid=:aid")?;
    // let rows = stmt.query_map(&[(":id", &"one")], |row| row.get(0))?;
    let iter = stmt.query_map(&[(":aid", &aid)], |row| {
      let cipher_pwd: Vec<u8> = row.get(2)?;
      let plain_pwd = self.cipher.decypt(&cipher_pwd).unwrap();
      Ok(Pwd {
        id: row.get(0)?,
        aid: row.get(1)?,
        password: String::from_utf8(plain_pwd).unwrap(),
        created: row.get(3)?,
      })
    })?;

    let mut pwds = iter.collect::<rusqlite::Result<Vec<Pwd>>>()?;
    pwds.sort_by(|a, b| b.created.partial_cmp(&a.created).unwrap());
    Ok(pwds)
    // Ok(iter.collect::<rusqlite::Result<Vec<Account>>>()?)
  }

  pub(crate) fn delete(&self, id: u32) -> TecResult<()> {
    let mut stmt = self.conn.prepare("DELETE FROM pwd WHERE aid = ?1")?;
    stmt.execute([id])?;
    Ok(())
  }
}
