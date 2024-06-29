use std::{
  fs::File,
  io::{Read, Write},
  path::Path,
};

use rand::{rngs::OsRng, RngCore};

use crate::{cipher::Argon2Cipher, common::TecResult};

pub struct KeyStore<P: AsRef<Path>> {
  path: P,
}

impl<P: AsRef<Path>> KeyStore<P> {
  pub fn new(path: P) -> Self {
    Self { path }
  }

  pub fn has_key(&self) -> bool {
    self.path.as_ref().exists()
    // self.path.exists()
  }

  pub fn set_key(&self, pwd: &[u8]) -> TecResult<()> {
    let mut key = [0u8; 32];
    OsRng.fill_bytes(&mut key);

    let cipher = Argon2Cipher::new(pwd);
    let enc = cipher.encrypt(&key)?;
    let mut file = File::create(&self.path)?;
    file.write_all(enc.as_bytes())?;
    Ok(())
  }

  pub fn get_key(&self, pwd: &[u8]) -> TecResult<Vec<u8>> {
    let mut file = File::open(&self.path)?;
    let mut enc = String::new();
    file.read_to_string(&mut enc)?;

    let cipher = Argon2Cipher::new(pwd);
    let key = cipher.decrypt(&enc)?;
    Ok(key)
  }
}
