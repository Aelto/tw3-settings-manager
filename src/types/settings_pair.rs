use std::fmt::Display;

#[derive(Clone)]
pub struct SettingsPair {
  pub key: String,
  pub value: String,

  pub is_selected: bool,
}

impl SettingsPair {
  pub fn new(key: String, value: String) -> Self {
    Self {
      key,
      value,
      is_selected: false,
    }
  }
}

impl Display for SettingsPair {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}={}", self.key, self.value)
  }
}
