use std::fs;
use std::io::{Error as IoError,ErrorKind};
use tauri::{App, Window, PageLoadPayload, Manager};
use super::config::config_plugin::ConfigPlugin;
use super::database::{DatabasePlugin, DbInstances};

pub struct  Application{
    app: App

}

impl Application{

    pub fn initialize() -> Result<(),IoError >{
        let local_dir = match tauri::api::path::local_data_dir(){
            Some(path) => path,
            None => return Err(IoError::new(ErrorKind::Other, "Local Directory is not Resolved"))
        };
        let config_dir = local_dir.join("sfm");
        if !config_dir.exists(){
            fs::create_dir(&config_dir)?
        }
        Ok(())
    }

    pub fn bootstrap() -> Self{
      Self{
        app :tauri::Builder::default()
            .plugin(ConfigPlugin::new())
            .plugin(DatabasePlugin::new())
            .on_page_load(Self::page_load_handler)
            .build(tauri::generate_context!())
            .expect("Failed to Build Application")
        }
    }

    fn page_load_handler( window: Window, _:PageLoadPayload ){
        window.emit("page-loaded",  window.label().to_string()).expect("failed to load Pages")
    }

    pub fn run(self){
        self.app.run(|app_handle, e| match e {
            tauri::Event::Ready => {
              tauri::async_runtime::spawn(async move {
                
              });
            },
            tauri::Event::CloseRequested{ label, .. } => {
                //let app_handle = app_handle.clone();
                tauri::async_runtime::block_on(async move {
                    let main_label = String::from("main");
                    if label.ne(&main_label) {
                        return;    
                    }
                    let db_instance = app_handle.state::<DbInstances>().inner().lock().await;
                    if !db_instance.is_closed(){
                        db_instance.close().await;
                    }
                });
            },
            tauri::Event::ExitRequested { window_label, .. } => {
                //let app_handle = app_handle.clone();
                tauri::async_runtime::block_on(async move {
                    let main_label = String::from("main");
                    if window_label.ne(&main_label) {
                        return;    
                    }
                    let db_instance = app_handle.state::<DbInstances>().inner().lock().await;
                    if !db_instance.is_closed(){
                        db_instance.close().await;
                    }
                });
            }
            _ => (),
          });
    }
} 