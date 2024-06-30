mod clipboard;
mod time;

pub use clipboard::{copy_content, get_pasted_content};
pub use time::{current_millis, millis2string};
