use std::collections::HashMap;
use std::fmt::Display;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use egui::CollapsingHeader;
use egui::Grid;
use egui::Ui;
use ini::Ini;

use super::SettingsCollection;
use super::SettingsPair;
use super::SettingsSection;

#[derive(Default)]
pub struct Settings {
  pub sections: HashMap<String, SettingsSection>,

  pub path: PathBuf,
  pub name: String,
  pub is_selected: bool,
}

impl Settings {
  pub fn from_settings_file(path: &PathBuf) -> Self {
    let name = path
      .file_name()
      .expect("Failed to get the settings file's name");

    let conf =
      Ini::load_from_file(path).expect("Failed to load ini configuration from user.settings file");

    let mut s = Self {
      path: path.clone(),
      sections: HashMap::new(),
      is_selected: false,
      name: name.to_str().unwrap().to_owned(),
    };

    for (section, properties) in &conf {
      if let Some(section) = section {
        let mut section = SettingsSection::new(section.to_owned(), HashMap::new());

        for (key, value) in properties.iter() {
          section.insert(SettingsPair::new(key.to_owned(), value.to_owned()))
        }

        s.add_section(section);
      }
    }

    s
  }

  pub fn add_section(&mut self, section: SettingsSection) {
    match self.sections.get_mut(&section.name) {
      Some(current) => {
        current.consume(section);
      }
      None => {
        self.sections.insert(section.name.clone(), section);
      }
    }
  }

  pub fn select(&mut self) {
    self.is_selected = true;

    for section in &mut self.sections.values_mut() {
      section.select();
    }
  }

  pub fn unselect(&mut self) {
    self.is_selected = false;

    for section in &mut self.sections.values_mut() {
      section.unselect();
    }
  }

  pub fn adjust_self_selected_from_pairs(&mut self) {
    self.is_selected = self.sections.values().all(|pair| pair.is_selected);
  }

  pub fn has_any_child_selected(&self) -> bool {
    self.sections.values().any(|c| c.has_any_child_selected())
  }

  pub fn into_only_selected(&self) -> Self {
    Self {
      path: self.path.clone(),
      is_selected: true,
      name: self.name.clone(),
      sections: self
        .sections
        .values()
        .filter(|section| section.has_any_child_selected())
        .map(|section| (section.name.clone(), section.into_only_selected()))
        .collect(),
    }
  }

  pub fn remove_selected(&mut self) {
    self
      .sections
      .drain_filter(|_, section| section.remove_selected());
  }

  pub fn import_selected(&mut self, collection: &SettingsCollection) {
    for settings in collection
      .settings
      .iter()
      .filter(|s| s.has_any_child_selected())
    {
      let mut imports = settings.into_only_selected();

      for section in imports.sections.into_values() {
        self.add_section(section);
      }
    }
  }

  pub fn export_selected(&self, collection: &mut SettingsCollection, export_name: &str) {
    if export_name.is_empty() {
      return;
    }

    let mut exports = self.into_only_selected();

    exports.name = export_name.to_owned();

    if let Err(_) = fs::create_dir_all("settings-collection") {
      // directory already exists
    }

    fs::write(
      Path::new("settings-collection").join(export_name),
      exports.to_string(),
    )
    .expect("Failed to write settings to collection");

    collection.settings.push(exports);
  }

  pub fn draw(&mut self, ui: &mut Ui, filter: &str) {
    let mut should_update_self_selected = false;

    // makes the scrollbar take all the available width
    ui.set_min_width(ui.available_width());

    for section in &mut self.sections.values_mut() {
      if !filter.is_empty() && !section.name.contains(&filter) {
        continue;
      }

      ui.horizontal(|ui| {
        if ui.checkbox(&mut section.is_selected, "").clicked() {
          should_update_self_selected = true;

          if section.is_selected {
            section.select();
          } else {
            section.unselect();
          }
        }

        CollapsingHeader::new(&section.name)
          .default_open(false)
          .show(ui, |ui| {
            Grid::new(&section.name).show(ui, |ui| {
              let mut should_update_section_selected = false;

              for pair in &mut section.pairs.values_mut() {
                if ui.checkbox(&mut pair.is_selected, "").clicked() {
                  should_update_section_selected = true;
                  should_update_self_selected = true;
                }

                ui.label(&pair.key);
                ui.label(&pair.value);
                ui.end_row();
              }

              if should_update_section_selected {
                section.adjust_self_selected_from_pairs();
              }
            });
          });
      });
    }

    if should_update_self_selected {
      self.adjust_self_selected_from_pairs();
    }
  }
}

impl Display for Settings {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for section in self.sections.values() {
      writeln!(f, "{section}")?;
    }

    Ok(())
  }
}
