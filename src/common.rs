use std::string::FromUtf8Error;

#[derive(thiserror::Error, Debug)]
pub enum TecError {
  #[error("invalid length - {0}")]
  InvalidLength(#[from] crypto_common::InvalidLength),
  #[error("aes error - {0}")]
  AESError(#[from] aes_gcm_siv::aead::Error),
  #[error("argon2 error - {0}")]
  Argon2Error(#[from] argon2::Error),
  #[error("fmt error")]
  FmtError(#[from] core::fmt::Error),
  #[error("parse int error")]
  ParseIntError(#[from] std::num::ParseIntError),
  #[error("IO error - {0}")]
  IOError(#[from] std::io::Error),
  #[error("sqlite error - {0}")]
  SqliteError(#[from] rusqlite::Error),
  #[error("csv error - {0}")]
  CSVError(#[from] csv::Error),
  #[error("from utf8 error - {0}")]
  FromUtf8Error(#[from] FromUtf8Error),
  #[error("var error - {0}")]
  VarError(#[from] std::env::VarError),
  #[error("clipboard paste error - {0}")]
  ClipboardPasteError(#[from] wl_clipboard_rs::paste::Error),
  #[error("clipboard copy error - {0}")]
  ClipboardCopyError(#[from] wl_clipboard_rs::copy::Error),
  #[error("invalid input")]
  InvalidInput,
  #[error("password not match")]
  PasswordNotMatch,
}

pub type TecResult<T> = Result<T, TecError>;
