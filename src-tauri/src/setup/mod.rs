//use std::fs;
use std::ops::Deref;
use serde::{Deserialize, Serialize};
use tauri::async_runtime::Mutex;
use std::sync::Arc;
use tauri::{plugin::Plugin, AppHandle, Manager, Runtime, State, Invoke, PageLoadPayload, Window};


#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct SetupDetails {
  localdir: String
}

impl SetupDetails {
  pub fn new(localdir: String) -> Self {
    Self{
      localdir 
    }
  }
}

type TSetupData = Arc<Mutex<SetupDetails>>;

#[tauri::command]
async fn get_setup(state: State<'_, TSetupData>) -> Result<serde_json::Value, String> {
  let setup = state.inner().lock().await;
  let setup_json = serde_json::to_value(setup.clone()).expect("Error While Parsing Config");
  Ok(setup_json)
}

pub struct AppSetUpPlugin<R: Runtime> {
  invoke_handler: Box<dyn Fn(Invoke<R>) + Send + Sync>,
}

impl<R: Runtime> AppSetUpPlugin<R> {

  pub fn init() -> Self {
    //println!("AppSetUp Plugin init");
    Self {
      invoke_handler: Box::new(tauri::generate_handler![get_setup]),
    }
  }
}

impl<R: Runtime> Plugin<R> for AppSetUpPlugin<R> {

  fn name(&self) -> &'static str {
    "appsetup"
  }

  fn on_page_load(&mut self, window: Window<R>, _payload: PageLoadPayload) {
    let setup = window.state::<TSetupData>().inner().blocking_lock().clone();
    window.emit("setup-init", setup).unwrap();
  }

  fn initialize(&mut self, app_handle: &AppHandle<R>, _config: serde_json::Value) -> tauri::plugin::Result<()> {
    let path_base = tauri::api::path::app_dir(app_handle.config().deref()).unwrap();
    //println!("path_base {:?}",&path_base);
    if tauri::api::dir::is_dir(&path_base).is_err() {
        std::fs::create_dir(&path_base).unwrap();
    }
    let setup_details: TSetupData = Arc::new(Mutex::new(SetupDetails::new(path_base.as_path().display().to_string())));
    app_handle.manage(setup_details); 
    Ok(())
  }

  fn initialization_script(&self) -> Option<String> {
    None
  }

  fn extend_api(&mut self, message: Invoke<R>) {
    (self.invoke_handler)(message)
  }

}

