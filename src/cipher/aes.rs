use aes_gcm_siv::{aead::Aead, Aes256GcmSiv, KeyInit, Nonce};
use rand::RngCore;

use crate::common::TecResult;

use super::NONCE_LEN;

pub struct AesCipher {
  aes: Aes256GcmSiv,
}

impl AesCipher {
  pub fn from_slice(key: &[u8]) -> TecResult<Self> {
    Ok(Self {
      aes: Aes256GcmSiv::new_from_slice(key)?,
    })
  }

  pub fn encypt(&self, plaintext: &[u8]) -> TecResult<Vec<u8>> {
    let mut nonce = [0u8; NONCE_LEN];
    rand::thread_rng().fill_bytes(&mut nonce);

    let ciphertext = self.aes.encrypt(Nonce::from_slice(&nonce), plaintext)?;
    Ok([&nonce[..], ciphertext.as_slice()].concat().to_vec())
  }

  pub fn decypt(&self, enc: &[u8]) -> TecResult<Vec<u8>> {
    let nonce = &enc[..NONCE_LEN];
    let ciphertext = &enc[NONCE_LEN..];

    let plaintext = self.aes.decrypt(Nonce::from_slice(nonce), ciphertext)?;
    Ok(plaintext)
  }
}
