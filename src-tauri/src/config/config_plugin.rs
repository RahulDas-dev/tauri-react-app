use tauri::{plugin::Plugin, Runtime, Invoke, State, Window, PageLoadPayload, Manager};
use tauri::async_runtime::Mutex;
use std::sync::Arc;
use super::AppConfig;


type Tconfig = Arc<Mutex<AppConfig>>;
pub struct ConfigPlugin<R: Runtime> {
    invoke_handler: Box<dyn Fn(Invoke<R>) + Send + Sync>,
}

#[tauri::command]
async fn save_config( config: State<'_, Tconfig>)-> Result<(),String>{
    println!("save config requested");
    let cfg = config.inner().lock().await;
    cfg.save_config().expect("Error While Saving Config");
    Ok(())
}

#[tauri::command]
async fn get_config( config: State<'_, Tconfig>)-> Result<serde_json::Value, String>{
    let cfg = config.inner().lock().await;
    let config_json =  serde_json::to_value(cfg.clone()).expect("Error While Parsing Config");
    Ok(config_json)
}

#[tauri::command]
async fn change_theme( config: State<'_, Tconfig>, theme: String) -> Result<(), String>{
    let mut cfg = config.inner().lock().await;
    cfg.change_theme(theme);
    Ok(())
}

#[tauri::command]
async fn change_dimension( config: State<'_, Tconfig>, height:f64, width:f64) -> Result<(), String>{
    let mut cfg = config.inner().lock().await;
    cfg.change_dimension(height, width);
    Ok(())
}


impl<R: Runtime> ConfigPlugin<R> {

    pub fn new()-> Self {
        Self {
            invoke_handler: Box::new(tauri::generate_handler![save_config, get_config,change_dimension,change_theme])
        }
    }
}


impl<R: Runtime> Plugin<R> for ConfigPlugin<R> {
  
    fn name(&self) -> &'static str {
      "config"
    }
  
    fn on_page_load(&mut self, window: Window<R>, _payload: PageLoadPayload) {
        window.emit("config-init", "hi").unwrap();
    }
  
    fn initialize(&mut self, app: &tauri::AppHandle<R>, _config: serde_json::Value) -> tauri::plugin::Result<()> {
      let app_config_state : Tconfig = Arc::new(Mutex::new(AppConfig::new()));
      app.manage(app_config_state);
      Ok(())
    }
  
    fn initialization_script(&self) -> Option<String> {
      None
    }
  
    fn created(&mut self, window: Window<R>) {
        let state = window.state::<Arc<Mutex<AppConfig>>>().inner().blocking_lock();
        window.set_resizable(true).expect("Error While seeting resizable");
        window.set_decorations(true).expect("Error While seeting resizable");
        window.set_skip_taskbar(false).expect("Error While seeting resizable");
        window.set_position(state.get_position()).expect("Error While seeting resizable");
        window.set_size(state.get_size()).expect("Error While seeting size");
        window.set_always_on_top(false).expect("Error While always on top");
    }

    fn extend_api(&mut self, message: Invoke<R>) {
      (self.invoke_handler)(message)
    }   
}