use super::config::ConfigPlugin;
use super::database::{DatabasePlugin, DbInstances};
use super::dialog::DialogPlugin;
use std::fs;
use std::io::{Error as IoError, ErrorKind};
use tauri::{App, Manager, PageLoadPayload, Window, WindowEvent};

pub struct Application {
  app: App,
}

impl Application {
  pub fn initialize() -> Result<(), IoError> {
    let local_dir = match tauri::api::path::local_data_dir() {
      Some(path) => path,
      None => return Err(IoError::new(ErrorKind::Other, "Local Directory is not Resolved")),
    };
    let config_dir = local_dir.join("sfm");
    if !config_dir.exists() {
      fs::create_dir(&config_dir)?
    }
    Ok(())
  }

  pub fn bootstrap() -> Self {
    Self {
      app: tauri::Builder::default()
        .plugin(ConfigPlugin::new())
        .plugin(DatabasePlugin::new())
        .plugin(DialogPlugin::new())
        .on_page_load(Self::page_load_handler)
        .build(tauri::generate_context!())
        .expect("Failed to Build Application"),
    }
  }

  fn page_load_handler(window: Window, _: PageLoadPayload) {
    window
      .emit("page-loaded", window.label().to_string())
      .expect("failed to load Pages")
  }

  pub fn run(self) {
    self.app.run(|app_handle, e| match e {
      tauri::RunEvent::Ready => {
        tauri::async_runtime::spawn(async move {});
      }
      tauri::RunEvent::WindowEvent { label, event, .. }=>{
        if label.ne("main") {
          return;
        }
        match event {
          WindowEvent::CloseRequested {  .. } => {
            tauri::async_runtime::block_on(async move {
              let db_instance = app_handle.state::<DbInstances>().inner().lock().await;
            if !db_instance.is_closed() {
                db_instance.close().await;
            }
          })
        }
          _ => {}
        }
      }
      tauri::RunEvent::ExitRequested { .. } => {
        //let app_handle = app_handle.clone();
        tauri::async_runtime::block_on(async move {
          let db_instance = app_handle.state::<DbInstances>().inner().lock().await;
          if !db_instance.is_closed() {
            db_instance.close().await;
          }
        });
      }
      _ => (),
    });
  }
}
