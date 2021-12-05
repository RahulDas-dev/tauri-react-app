use tauri::{Builder, Wry, Window, PageLoadPayload,};


// use super::config::AppConfig;
use super::config::config_plugin::ConfigPlugin;


pub struct  Application{
    app: Builder<Wry>
}



impl Application{

    pub fn bootstrap() -> Self{
      Self{
        app :tauri::Builder::default()
            .plugin(ConfigPlugin::new())
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