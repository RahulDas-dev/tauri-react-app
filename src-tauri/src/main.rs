#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

pub mod application;
pub mod setup;
pub mod config;
pub mod database;
pub mod dialog;

use application::Application;

fn main() {
  Application::bootstrap().run();
}
