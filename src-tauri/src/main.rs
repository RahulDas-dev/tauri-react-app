#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

pub mod config;
pub mod application;
use application::Application;


fn main() {

  Application::bootstrap().run();

}
