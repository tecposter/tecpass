use aes_gcm_siv::{aead::Aead, Aes256GcmSiv, KeyInit, Nonce};
use argon2::Argon2;
use rand::RngCore;

use crate::{common::TecResult, hex};

use super::NONCE_LEN;

#[allow(dead_code)]
const SALT_LEN: usize = 32;
#[allow(dead_code)]
const ENC_KEY_LEN: usize = 32;

pub struct Argon2Cipher<'a> {
  pwd: &'a [u8],
}

impl<'a> Argon2Cipher<'a> {
  pub fn new(pwd: &'a [u8]) -> Self {
    Self { pwd }
  }

  pub fn encrypt(&self, plaintext: &[u8]) -> TecResult<String> {
    let mut salt = [0u8; SALT_LEN];
    let mut nonce = [0u8; NONCE_LEN];
    let mut enc_key = [0u8; ENC_KEY_LEN];

    rand::thread_rng().fill_bytes(&mut salt);
    rand::thread_rng().fill_bytes(&mut nonce);
    Argon2::default().hash_password_into(self.pwd, &salt, &mut enc_key)?;

    let cipher = Aes256GcmSiv::new_from_slice(&enc_key)?;
    let ciphertext = cipher.encrypt(Nonce::from_slice(&nonce), plaintext)?;

    let enc = [&salt[..], &nonce[..], ciphertext.as_slice()].concat();
    Ok(hex::encode(&enc)?)
  }

  pub fn decrypt(&self, enc_str: &str) -> TecResult<Vec<u8>> {
    let enc = hex::decode(enc_str)?;

    let salt = &enc[..SALT_LEN];
    let nonce = &enc[SALT_LEN..(SALT_LEN + NONCE_LEN)];
    let ciphertext = &enc[(SALT_LEN + NONCE_LEN)..];

    let mut enc_key = [0u8; ENC_KEY_LEN];
    Argon2::default().hash_password_into(self.pwd, salt, &mut enc_key)?;
    let cipher = Aes256GcmSiv::new_from_slice(&enc_key)?;
    let plaintext = cipher.decrypt(Nonce::from_slice(nonce), ciphertext)?;
    Ok(plaintext)
  }
}
