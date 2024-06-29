use unicode_width::UnicodeWidthStr;

#[derive(Debug, Default, Clone)]
pub struct Account {
  pub id: u32,
  pub url: String,
  pub username: String,
  // pub password: String,
  pub created: usize,
  pub changed: usize,
}

impl Account {
  pub fn to_string(&self) -> String {
    format!("{} - {} - {}", self.id, self.url, self.username)
  }

  pub fn id(&self) -> u32 {
    self.id
  }

  pub fn url(&self) -> &str {
    &self.url
  }

  pub fn username(&self) -> &str {
    &self.username
  }

  pub fn id_len(&self) -> u16 {
    let id_str: &str = &self.id.to_string();
    UnicodeWidthStr::width(id_str) as u16
  }

  pub fn url_len(&self) -> u16 {
    UnicodeWidthStr::width(&self.url[..]) as u16
  }

  pub fn username_len(&self) -> u16 {
    UnicodeWidthStr::width(&self.username[..]) as u16
  }
}

impl AsRef<Account> for Account {
  fn as_ref(&self) -> &Account {
    self
  }
}

#[derive(Debug)]
pub struct Pwd {
  pub id: u32,
  pub aid: u32,
  pub password: String,
  pub created: usize,
}
