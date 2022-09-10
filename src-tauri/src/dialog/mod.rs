use std::fs;
use std::sync::mpsc;
use std::{ffi::OsStr, path::PathBuf};
use tauri::plugin::Plugin; 
use tauri::{Invoke, Runtime};
use tauri::api::dialog::FileDialogBuilder;

#[tauri::command]
async fn open_input_dialog() -> Result<String, String> {
  let (tx, rx) = mpsc::channel::<PathBuf>();
  FileDialogBuilder::new().pick_folder(move |path| {
    match path {
      Some(path) => tx.send(path).unwrap(),
      None => tx.send(PathBuf::from("")).unwrap(),
    }
    return;
  });
  match rx.recv() {
    Ok(path) => {
      if path == PathBuf::from("") {
        return Ok("".to_string());
      }
      let entry_list = fs::read_dir(&path);
      if entry_list.is_err() {
        return Err(String::from("Error While Reading Selected Directory"));
      }
      let file_list = entry_list
        .unwrap()
        .filter_map(Result::ok)
        .map(|file| file.path())
        .map(|f| is_extension_valid(&f))
        .filter(|f| *f)
        .collect::<Vec<bool>>();
      if file_list.len() < 3 {
        return Err(String::from("Input Dir Should have minimum 3 image files"));
      }
      Ok(path.display().to_string())
    }
    Err(e) => {
      println!("Err Arm path");
      Err(e.to_string())
    }
  }
}

#[tauri::command]
async fn open_output_dialog() -> Result<String, String> {
  let (tx, rx) = mpsc::channel::<PathBuf>();
  FileDialogBuilder::new().pick_folder(move |path| {
    match path {
      Some(path) => tx.send(path).unwrap(),
      None => tx.send(PathBuf::from("")).unwrap(),
    }
    return;
  });
  match rx.recv() {
    Ok(path) => {
      if path == PathBuf::from("") {
        return Ok("".to_string());
      }
      let entry_list = fs::read_dir(&path);
      if entry_list.is_err() {
        return Err(String::from("Error While Reading Selected Directory"));
      }
      let file_list = entry_list
        .unwrap()
        .filter_map(Result::ok)
        .map(|file| file.path())
        .collect::<Vec<PathBuf>>();
      if file_list.len() > 0 {
        return Err(String::from("Output Dir should be empty"));
      }
      Ok(path.display().to_string())
    }
    Err(e) => Err(e.to_string()),
  }
}

fn is_extension_valid(path: &PathBuf) -> bool {
  for ext in ["jpg", "png", "jpeg"] {
    if path.extension() == Some(OsStr::new(ext)) {
      return true;
    }
  }
  return false;
}

pub struct DialogPlugin<R: Runtime> {
  invoke_handler: Box<dyn Fn(Invoke<R>) + Send + Sync>,
}

impl<R: Runtime> DialogPlugin<R> {
  pub fn new() -> Self {
    Self {
      invoke_handler: Box::new(tauri::generate_handler![open_input_dialog, open_output_dialog]),
    }
  }
}

impl<R: Runtime> Plugin<R> for DialogPlugin<R> {
  fn name(&self) -> &'static str {
    "dialog"
  }

  fn initialize(&mut self, _: &tauri::AppHandle<R>, _: serde_json::Value) -> tauri::plugin::Result<()> {
    Ok(())
  }

  fn extend_api(&mut self, message: Invoke<R>) {
    (self.invoke_handler)(message)
  }

  fn initialization_script(&self) -> Option<String> {
    None
  }

  fn created(&mut self, _: tauri::Window<R>) {}

  fn on_page_load(&mut self, _: tauri::Window<R>, _: tauri::PageLoadPayload) {}
}
