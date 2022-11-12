use std::fs::read_dir;
use std::path::Path;
use std::process::Command;

use egui::CollapsingHeader;
use egui::ScrollArea;
use egui::Ui;

use super::Settings;

pub struct SettingsCollection {
  pub settings: Vec<Settings>,
}

impl SettingsCollection {
  pub fn new() -> Self {
    Self {
      settings: Vec::new(),
    }
  }

  pub fn load_collection(&mut self) {
    self.settings.clear();

    if !Path::new("settings-collection").exists() {
      return;
    }

    for entry in
      read_dir("settings-collection").expect("Failed to read settings-collection directory")
    {
      let entry = entry.expect("Failed to read settings-collection entry");
      let path = entry.path();

      let setting = Settings::from_settings_file(&path);

      self.settings.push(setting);
    }
  }

  pub fn draw(&mut self, ui: &mut Ui, filter: &str) {
    ScrollArea::vertical()
      .id_source("collection")
      .show(ui, |ui| {
        for settings in &mut self.settings {
          ui.horizontal(|ui| {
            if ui.checkbox(&mut settings.is_selected, "").clicked() {
              if settings.is_selected {
                settings.select();
              } else {
                settings.unselect();
              }
            }

            ui.menu_button("...", |ui| {
              if cfg!(windows) {
                if ui.button("Reveal in File explorer").clicked() {
                  Command::new("explorer")
                    .arg(format!("/select,{}", &settings.path.to_str().unwrap()))
                    .spawn()
                    .expect("Failed to open file explorer at location");
                }
              }
            });

            CollapsingHeader::new(&settings.name).show(ui, |ui| {
              settings.draw(ui, filter);
            });
          });
        }
      });
  }
}
