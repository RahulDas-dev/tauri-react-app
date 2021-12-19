#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

pub mod application;
pub mod config;
pub mod database;
pub mod dialog;

use application::Application;

fn main() {
  Application::initialize().unwrap();
  Application::bootstrap().run();
}
