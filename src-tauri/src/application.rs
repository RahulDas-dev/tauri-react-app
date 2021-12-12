use std::fs;
use std::io::{Error as IoError,ErrorKind};
use tauri::{Builder, Wry, Window, PageLoadPayload};
// use super::config::AppConfig;
use super::config::config_plugin::ConfigPlugin;
use super::database::DatabasePlugin;

pub struct  Application{
    app: Builder<Wry>
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
        }
    }

    fn page_load_handler( window: Window, _:PageLoadPayload ){
        window.emit("page-loaded",  window.label().to_string()).expect("failed to load Pages")
    }

    pub fn run(self){
        self.app.run(tauri::generate_context!())
        .expect("error while running tauri application");
    }
}