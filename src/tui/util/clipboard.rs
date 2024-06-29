use std::io::Read;
use wl_clipboard_rs::{
  copy::{MimeType as CopyMimeType, Options, Source},
  paste::ClipboardType,
  paste::{get_contents, Error, MimeType, Seat},
};

use crate::common::TecResult;

pub fn get_pasted_content() -> TecResult<Option<String>> {
  let result = get_contents(ClipboardType::Regular, Seat::Unspecified, MimeType::Text);
  match result {
    Ok((mut pipe, _)) => {
      let mut contents = vec![];
      pipe.read_to_end(&mut contents)?;
      Ok(Some(String::from_utf8_lossy(&contents).into()))
      // println!("Pasted: {}", String::from_utf8_lossy(&contents));
    }

    Err(Error::NoSeats) | Err(Error::ClipboardEmpty) | Err(Error::NoMimeType) => {
      Ok(None)
      // The clipboard is empty or doesn't contain text, nothing to worry about.
    }

    Err(err) => Err(err)?,
  }
}

pub fn copy_content(bytes: &[u8]) -> TecResult<()> {
  let opts = Options::new();
  opts.copy(Source::Bytes(bytes.into()), CopyMimeType::Autodetect)?;
  Ok(())
}
