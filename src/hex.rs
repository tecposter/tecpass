use std::{fmt::Write, num::ParseIntError};

use crate::common::TecResult;

#[derive(thiserror::Error, Debug)]
pub enum HexError {
  #[error("fmt error")]
  FmtError(#[from] core::fmt::Error),
  #[error("parse int error")]
  ParseIntError(#[from] ParseIntError),
}

pub fn encode(bytes: &[u8]) -> TecResult<String> {
  let mut s = String::with_capacity(bytes.len() * 2);
  for &b in bytes {
    write!(&mut s, "{:02x}", b)?
  }
  Ok(s)
}

pub fn decode(s: &str) -> TecResult<Vec<u8>> {
  let res: Result<Vec<u8>, _> = (0..s.len())
    .step_by(2)
    .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
    .collect();
  Ok(res?)
}
