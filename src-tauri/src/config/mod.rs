mod config;

use config::WindowConfig;
use std::fs::File;
use std::ops::Deref;
use std::io::{prelude::*, Error};
use std::path::PathBuf;
use std::sync::Arc;
use tauri::async_runtime::Mutex;
use tauri::{plugin::Plugin, Invoke, Manager, PageLoadPayload, Runtime, State, Window};
use tauri::{api::path::{BaseDirectory, resolve_path},RunEvent, WindowEvent,Env };

type Tconfig = Arc<Mutex<WindowConfig>>;

pub struct ConfigPlugin<R: Runtime> {
  invoke_handler: Box<dyn Fn(Invoke<R>) + Send + Sync>,
  config_path: PathBuf 
}

#[tauri::command]
async fn get_config(config: State<'_, Tconfig>) -> Result<serde_json::Value, String> {
  let cfg = config.inner().lock().await;
  let config_json = serde_json::to_value(cfg.clone()).expect("Error While Parsing Config");
  Ok(config_json)
}

#[tauri::command]
async fn change_theme(config: State<'_, Tconfig>, theme: String) -> Result<(), String> {
  let mut cfg = config.inner().lock().await;
  cfg.change_theme(theme);
  Ok(())
}

impl<R: Runtime> ConfigPlugin<R> {
  pub fn new() -> Self {
    Self {
      invoke_handler: Box::new(tauri::generate_handler![get_config, change_theme]),
      config_path: PathBuf::default()

    }
  }

  pub fn save_config(&self, state: &WindowConfig, ) -> Result<(), Error> {
    let config_str = serde_json::to_string(state)?;
    File::create(&self.config_path)?.write_all(config_str.as_bytes())?;
    Ok(())
  }
}

impl<R: Runtime> Plugin<R> for ConfigPlugin<R> {
  fn name(&self) -> &'static str {
    "config"
  }

  fn on_page_load(&mut self, window: Window<R>, _payload: PageLoadPayload) {
    let config = window.state::<Tconfig>().inner().blocking_lock().clone();
    window.emit("config-init", config).unwrap();
  }

  fn initialize(&mut self, app: &tauri::AppHandle<R>, _config: serde_json::Value) -> tauri::plugin::Result<()> {

    let config_path = resolve_path(
      app.config().deref(),
      app.package_info().deref(),
      &Env::default(),
      "config.json",
      Some(BaseDirectory::App))
    .expect("Failed to resolve Config path");
    
    if !config_path.exists() {
      let config_default: WindowConfig = WindowConfig::default();
      let config_str = serde_json::to_string(&config_default)?;
      File::create(&config_path)?.write_all(config_str.as_bytes())?;
    }
    let app_config_state: Tconfig = Arc::new(Mutex::new(WindowConfig::new(&config_path)));
    app.manage(app_config_state);
    self.config_path = config_path.clone();
    Ok(())
  }

  fn initialization_script(&self) -> Option<String> {
    None
  }

  fn created(&mut self, window: Window<R>) {
    let label = String::from("main");
    if window.label().to_string().ne(&label) {
      return;
    }
    let state = window.state::<Arc<Mutex<WindowConfig>>>().inner().blocking_lock();
    window.set_resizable(true).unwrap();
    window.set_position(state.get_position()).unwrap();
    window.set_size(state.get_size()).unwrap();

    let colned = window.clone();

    window.on_window_event(move |e| match e {
      WindowEvent::Moved(position) => {
        let mut state = colned.state::<Arc<Mutex<WindowConfig>>>().inner().blocking_lock();
        state.change_position(position.x, position.y);
      },
      WindowEvent::Resized(size) => {
        let mut state = colned.state::<Arc<Mutex<WindowConfig>>>().inner().blocking_lock();
        state.change_dimension(size.width, size.height);
      },
      _ => {}
    });

    window.show().unwrap();
    window.set_focus().unwrap();
  }

  fn extend_api(&mut self, message: Invoke<R>) {
    (self.invoke_handler)(message)
  }

  fn on_event(&mut self, app: &tauri::AppHandle<R>, event: &RunEvent) {
    
    match event {
      RunEvent::WindowEvent{ label, event, ..} => {
        match event {
          WindowEvent::CloseRequested { .. } => {
            //api.prevent_close();
            let window = app.get_window(&label).unwrap();
            let state = window.state::<Arc<Mutex<WindowConfig>>>().inner().blocking_lock();
            self.save_config(&state).unwrap();
          },
          WindowEvent::Destroyed => {
            let window = app.get_window(&label).unwrap();
            let state = window.state::<Arc<Mutex<WindowConfig>>>().inner().blocking_lock();
            self.save_config(&state).unwrap();
          }
          _ => {}
        }
        
      },
      _ => {}
    }
  }
}
