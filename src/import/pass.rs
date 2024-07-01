use std::{
  fs::File,
  io::{self, BufRead},
  path::Path,
  rc::Rc,
  time::{SystemTime, UNIX_EPOCH},
};

use crate::{
  cipher::AesCipher,
  common::TecResult,
  db::sqlite_conn,
  model::{Account, Pwd},
  repo::{AccountRepo, PwdRepo},
};

pub fn import_pass_accounts<P: AsRef<Path>>(src_path: P, db_path: P, key: &[u8]) -> TecResult<()> {
  let conn = Rc::new(sqlite_conn(db_path)?);
  let aes_cipher = Rc::new(AesCipher::from_slice(key)?);

  let account_repo = AccountRepo::new(conn.clone(), aes_cipher.clone());
  let pwd_repo = PwdRepo::new(conn, aes_cipher);

  let src = File::open(src_path)?;
  let lines = io::BufReader::new(src).lines().flatten();

  let mut ending = 0;
  let mut index = 0;
  let mut count: u32 = 0;

  let mut name = "".to_string();
  let mut pwd = "".to_string();
  let mut login = "".to_string();
  let mut url = "".to_string();

  for line in lines {
    if index == 0 && line.starts_with("Name: ") {
      name = line[6..].trim().to_string();
    }
    if index == 1 {
      pwd = line.trim().to_string();
    }
    if index == 2 {
      if line.starts_with("login: ") {
        login = line[7..].trim().to_string();
      } else {
        login = "".to_string();
      }
    }

    if index == 3 {
      if line.starts_with("url: ") {
        url = line[5..].trim().to_string();
      } else {
        url = "".to_string();
      }
    }

    index += 1;
    if line.is_empty() {
      ending += 1;
      if ending == 2 {
        let aid = add_account(&account_repo, &pwd_repo, &name, &pwd, &login, &url)?;
        println!("{count}:{aid} \nname: {name} \npwd: {pwd} \nlogin: {login} \nurl: {url} \n\n");
        ending = 0;
        index = 0;
        count += 1;
      }
    }
  }
  println!("[[[[[[[[result]]]]]]]]");
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

fn add_account(
  account_repo: &AccountRepo,
  pwd_repo: &PwdRepo,
  pm_name: &str,
  pwd: &str,
  login: &str,
  pm_url: &str,
) -> TecResult<u32> {
  let url = {
    if pm_url.is_empty() {
      pm_name.to_string()
    } else {
      pm_url.to_string()
    }
  };
  let now = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .expect("Clock may have gone backwards")
    .as_millis() as usize;
  let aid = account_repo.add(&Account {
    id: 0,
    url: url.to_string(),
    username: login.to_string(),
    created: now,
    changed: now,
  })?;
  pwd_repo.add(&Pwd {
    id: 0,
    aid,
    password: pwd.to_string(),
    created: now,
  })?;
  Ok(aid)
}
