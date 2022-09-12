use serde::{Deserialize, Serialize};
use std::default::Default;
use std::fs::File;
use std::path::PathBuf;
use tauri::{PhysicalPosition, PhysicalSize, Position, Size};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct WindowConfig {
  workspace: String,
  theme: String,
  x: i32,
  y: i32,
  width: u32,
  height: u32,
}

impl Default for WindowConfig {
  fn default() -> Self {
    WindowConfig {
      workspace: String::from(""),
      theme: String::from("White"),
      x: 0i32,
      y: 0i32,
      width: 800u32,
      height: 600u32,
    }
  }
}

impl WindowConfig {
  pub fn new(config_path: &PathBuf) -> Self {
    let config_file = File::open(config_path).expect("Failed To Read Config.json");
    serde_json::from_reader(config_file).expect("Failed To Parse Config.json")
  }

  pub fn change_workspace(&mut self, workspace: String) {
    self.workspace = workspace.clone();
  }

  pub fn change_theme(&mut self, theme: String) {
    self.theme = theme.clone();
  }

  pub fn change_dimension(&mut self, width: u32, height: u32) {
    self.width = width;
    self.height = height;
  }

  pub fn change_position(&mut self, x: i32, y: i32) {
    self.x = x;
    self.y = y;
  }

  pub fn get_position(&self) -> Position {
    return Position::Physical(PhysicalPosition { x: self.x, y: self.y });
  }

  pub fn get_size(&self) -> Size {
    return Size::Physical(PhysicalSize {
      width: self.width,
      height: self.height,
    });
  }

}
