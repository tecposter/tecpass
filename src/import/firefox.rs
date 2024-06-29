use std::{path::Path, rc::Rc, usize};

use crate::{
  cipher::AesCipher,
  common::TecResult,
  db::{sqlite_conn, KeyStore},
  model::{Account, Pwd},
  repo::{AccountRepo, PwdRepo},
};

pub fn import_firefox_accounts<P: AsRef<Path>>(
  csv_path: P,
  // key_path: P,
  db_path: P,
  key: &[u8],
  // pwd: &str,
  // conn: &rusqlite::Connection,
) -> TecResult<()> {
  let conn = Rc::new(sqlite_conn(db_path)?);
  let mut rdr = csv::Reader::from_path(csv_path)?;
  // let pwd_repo = get_pwd_repo(key_path, pwd, conn)?;

  // let key_store = KeyStore::new(key_path);
  // let aes_key = key_store.get_key(pwd.as_bytes())?;
  let aes_cipher = Rc::new(AesCipher::from_slice(key)?);

  let account_repo = AccountRepo::new(conn.clone(), aes_cipher.clone());
  let pwd_repo = PwdRepo::new(conn, aes_cipher);

  // let headers = rdr.headers()?;
  // println!("{:?}", headers);

  println!("======");
  println!("import");
  println!("======");
  for res in rdr.records() {
    let record = res?;
    let url = record.get(0).unwrap();
    let username = record.get(1).unwrap();
    let password = record.get(2).unwrap();
    let http_realm = record.get(3).unwrap();
    let from_action_origin = record.get(4).unwrap();
    let guid = record.get(5).unwrap();
    let time_created = record.get(6).unwrap();
    let time_last_used = record.get(7).unwrap();
    let time_password_changed = record.get(8).unwrap();

    let created = time_created.parse::<usize>()?;
    let changed = time_password_changed.parse::<usize>()?;

    let aid = account_repo.add(&Account {
      id: 0,
      url: url.to_string(),
      username: username.to_string(),
      created,
      changed,
    })?;

    pwd_repo.add(&Pwd {
      id: 0,
      aid,
      password: password.to_string(),
      created,
    })?;

    println!("{aid}: ");
    println!(" - url: {url}");
    println!(" - username: {username}");
    println!(" - created: {created}");
    println!(" - changed: {changed}");
    println!(" - password: {password}");
    println!(" - (x) http_realm: {http_realm}");
    println!(" - (x) from_action_origin: {from_action_origin}");
    println!(" - (x) guid: {guid}");
    println!(" - (x) time_last_used: {time_last_used}");
  }

  println!("result");
  let accounts = account_repo.all()?;
  for account in accounts {
    println!(" - {account:?}");
    let pwds = pwd_repo.query(account.id)?;
    for pwd in pwds {
      println!(" - - {pwd:?}");
    }
  }

  Ok(())
}

fn get_pwd_repo<'a, P: AsRef<Path>>(
  key_path: P,
  pwd: &str,
  conn: Rc<rusqlite::Connection>,
) -> TecResult<PwdRepo> {
  // let key_store = KeyStore::new(key_path);
  let key_store = KeyStore::new(key_path);
  let aes_key = key_store.get_key(pwd.as_bytes())?;
  let aes_cipher = Rc::new(AesCipher::from_slice(&aes_key)?);
  let pwd_repo = PwdRepo::new(conn, aes_cipher);

  Ok(pwd_repo)
}
