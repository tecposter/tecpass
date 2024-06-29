use std::path::Path;

use rusqlite::Connection;

use crate::common::TecResult;

pub fn sqlite_conn<P: AsRef<Path>>(path: P) -> TecResult<Connection> {
  let conn = Connection::open(path)?;
  conn.execute(
    r#"
    CREATE TABLE if not exists account (
        id    INTEGER PRIMARY KEY,
        url  BLOB NOT NULL,
        username BLOB NOT NULL,
        created INTEGER,
        changed INTEGER
    );
    "#,
    (),
  )?;
  conn.execute(
    r#"
    CREATE TABLE if not exists pwd (
        id INTEGER PRIMARY KEY,
        aid INTEGER NOT NULL,
        password BLOB NOT NULL,
        created INTEGER
    );
    "#,
    (),
  )?;
  Ok(conn)
}
