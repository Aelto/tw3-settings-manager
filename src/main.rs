// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![feature(hash_drain_filter)]

use std::path::PathBuf;

use clipboard_win::set_clipboard_string;
use eframe::egui;
use egui::ScrollArea;
use egui::Ui;
use egui::Vec2;
use types::Settings;
use types::SettingsCollection;

mod types;

fn main() {
  let options = eframe::NativeOptions {
    initial_window_size: Some(Vec2::new(1200.0, 750.0)),
    ..Default::default()
  };
  eframe::run_native(
    "Witcher 3 - Settings Manager",
    options,
    Box::new(|_cc| Box::new(MyApp::default())),
  );
}

struct MyApp {
  /// Path to the `user.settings` file.
  settings_path: PathBuf,

  settings: Settings,
  collection: SettingsCollection,

  section_filter: String,
  export_name: String,
}

impl MyApp {
  fn load_settings(&mut self) {
    self.settings = Settings::from_settings_file(&self.settings_path);
  }
}

impl Default for MyApp {
  fn default() -> Self {
    let mut output = Self {
      settings_path: dirs::document_dir()
        .unwrap()
        .join("The Witcher 3/user.settings"),
      collection: SettingsCollection::new(),
      section_filter: Default::default(),
      settings: Default::default(),
      export_name: Default::default(),
    };

    output.load_settings();
    output.collection.load_collection();
    output
  }
}

impl eframe::App for MyApp {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
      ui.horizontal(|ui| {
        if ui.button("Load settings file").clicked() {
          self.load_settings();
        }

        if ui.button("Load settings collection").clicked() {
          self.collection.load_collection();
        }

        ui.add(egui::TextEdit::singleline(&mut self.section_filter).hint_text("Search"));
      });
    });

    if !self.collection.settings.is_empty() {
      egui::SidePanel::right("collection_panel")
        .min_width(ctx.used_size().x * 0.5)
        .resizable(false)
        .show(ctx, |ui| {
          self.draw_collection_column(ui);
        });
    }

    egui::CentralPanel::default().show(ctx, |ui| {
      self.draw_preview_column(ui);
    });
  }
}

impl MyApp {
  fn draw_preview_column(&mut self, ui: &mut Ui) {
    ui.heading("Output preview");

    if !self.settings.sections.is_empty() {
      ui.horizontal(|ui| {
        ui.add(egui::TextEdit::singleline(&mut self.export_name).hint_text("my-awesome-preset"));

        if ui.button("Export into collection").clicked() {
          self
            .settings
            .export_selected(&mut self.collection, &self.export_name);

          self.export_name.clear();
        }
      });

      ui.separator();
    }

    ui.horizontal(|ui| {
      if ui.button("Clear").clicked() {
        self.settings.sections.clear();
      }

      if ui.button("Remove selected").clicked() {
        self.settings.remove_selected();
      }

      if ui.button("Copy to clipboard").clicked() {
        let clipboard_content = if self.settings.has_any_child_selected() {
          self.settings.into_only_selected().to_string()
        } else {
          self.settings.to_string()
        };

        set_clipboard_string(&clipboard_content).expect("Failed to write clipboard content");
      }

      ui.add_enabled_ui(false, |ui| {
        ui.button("Apply to user.settings")
          .on_disabled_hover_text("Currently not possible while the tool is in development");
      });
    });

    ScrollArea::vertical().id_source("preview").show(ui, |ui| {
      self.settings.draw(ui, &self.section_filter);
    });
  }

  fn draw_collection_column(&mut self, ui: &mut Ui) {
    ui.heading("Settings collection");

    ui.horizontal(|ui| {
      if ui.button("Import selected").clicked() {
        self.settings.import_selected(&self.collection);
      }
    });

    self.collection.draw(ui, &self.section_filter);
  }
}
