use std::collections::HashMap;
use std::fmt::Display;

use super::SettingsPair;

#[derive(Clone)]
pub struct SettingsSection {
  pub name: String,
  pub pairs: HashMap<String, SettingsPair>,

  pub is_selected: bool,
}

impl SettingsSection {
  pub fn new(name: String, pairs: HashMap<String, SettingsPair>) -> Self {
    Self {
      name,
      pairs,
      is_selected: false,
    }
  }

  pub fn insert(&mut self, pair: SettingsPair) {
    self.pairs.insert(pair.key.clone(), pair);
  }

  pub fn consume(&mut self, other: SettingsSection) {
    self.pairs.extend(other.pairs);
  }

  pub fn select(&mut self) {
    self.is_selected = true;

    for pair in &mut self.pairs.values_mut() {
      pair.is_selected = true;
    }
  }

  pub fn unselect(&mut self) {
    self.is_selected = false;

    for pair in &mut self.pairs.values_mut() {
      pair.is_selected = false;
    }
  }

  pub fn adjust_self_selected_from_pairs(&mut self) {
    self.is_selected = self.pairs.values().all(|pair| pair.is_selected);
  }

  pub fn has_any_child_selected(&self) -> bool {
    self.pairs.values().any(|c| c.is_selected)
  }

  pub fn into_only_selected(&self) -> Self {
    Self {
      is_selected: true,
      name: self.name.clone(),
      pairs: self
        .pairs
        .values()
        .filter(|pair| pair.is_selected)
        .map(|pair| (pair.key.clone(), pair.clone()))
        .collect(),
    }
  }

  /// returns whether this section is empty, so the function can be used in
  /// `drain_filter`s
  pub fn remove_selected(&mut self) -> bool {
    self.pairs.drain_filter(|_, pair| pair.is_selected);

    self.pairs.is_empty()
  }
}

impl Display for SettingsSection {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "[{}]", self.name)?;

    for pair in self.pairs.values() {
      writeln!(f, "{pair}")?;
    }

    Ok(())
  }
}
