mod config;

use config::AppConfig;
use std::fs::File;
use std::io::{prelude::*, Error, ErrorKind};
use std::path::PathBuf;
use std::sync::Arc;
use tauri::async_runtime::Mutex;
use tauri::WindowEvent;
use tauri::{plugin::Plugin, Invoke, Manager, PageLoadPayload, Runtime, State, Window};

type Tconfig = Arc<Mutex<AppConfig>>;
pub struct ConfigPlugin<R: Runtime> {
  invoke_handler: Box<dyn Fn(Invoke<R>) + Send + Sync>,
}

/* #[tauri::command]
async fn save_config( config: State<'_, Tconfig>)-> Result<(),String>{
    println!("save config requested");
    let cfg = config.inner().lock().await;
    cfg.save_config().expect("Error While Saving Config");
    Ok(())
} */

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
    }
  }

  fn resolve_config_path() -> Result<PathBuf, Error> {
    let config_path = match tauri::api::path::local_data_dir() {
      Some(path) => path.join("sfm").join("config.join"),
      None => return Err(Error::new(ErrorKind::Other, "Local Dir not Resolved")),
    };
    if !config_path.exists() {
      let config_default: AppConfig = AppConfig::default();
      let config_str = serde_json::to_string(&config_default)?;
      File::create(&config_path)?.write_all(config_str.as_bytes())?;
    }
    Ok(config_path)
  }

  pub fn save_config(state: &AppConfig) -> Result<(), Error> {
    let config_path = match Self::resolve_config_path() {
      Ok(path) => path,
      Err(error) => panic!("{}", error),
    };
    let config_str = serde_json::to_string(state)?;
    File::create(&config_path)?.write_all(config_str.as_bytes())?;
    Ok(())
  }
}

impl<R: Runtime> Plugin<R> for ConfigPlugin<R> {
  fn name(&self) -> &'static str {
    "config"
  }

  fn on_page_load(&mut self, window: Window<R>, _payload: PageLoadPayload) {
    let config = window.state::<Arc<Mutex<AppConfig>>>().inner().blocking_lock().clone();
    window.emit("config-init", config).unwrap();
  }

  fn initialize(&mut self, app: &tauri::AppHandle<R>, _config: serde_json::Value) -> tauri::plugin::Result<()> {
    let config_path = Self::resolve_config_path()?;
    let app_config_state: Tconfig = Arc::new(Mutex::new(AppConfig::new(&config_path)));
    //println!("{:?}", app_config_state.clone().blocking_lock());
    app.manage(app_config_state);
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
    let state = window.state::<Arc<Mutex<AppConfig>>>().inner().blocking_lock();
    window.set_resizable(true).unwrap();
    window.set_position(state.get_position()).unwrap();
    window.set_size(state.get_size()).unwrap();

    let colned = window.clone();

    window.on_window_event(move |e| match e {
      WindowEvent::Moved(position) => {
        let mut state = colned.state::<Arc<Mutex<AppConfig>>>().inner().blocking_lock();
        state.change_position(position.x, position.y);
      }
      WindowEvent::Resized(size) => {
        //println!("window Resized {:?}", size);
        let mut state = colned.state::<Arc<Mutex<AppConfig>>>().inner().blocking_lock();
        state.change_dimension(size.width, size.height);
      }
      WindowEvent::CloseRequested | WindowEvent::Destroyed => {
        let state = colned.state::<Arc<Mutex<AppConfig>>>().inner().blocking_lock();
        //println!("{:?}", state.clone());
        Self::save_config(&state).unwrap();
      }
      _ => {}
    });

    window.show().unwrap();
    window.set_focus().unwrap();
  }

  fn extend_api(&mut self, message: Invoke<R>) {
    (self.invoke_handler)(message)
  }

  /* fn on_event(&mut self, app: &AppHandle<R>, event: &tauri::Event) {

  } */
}
