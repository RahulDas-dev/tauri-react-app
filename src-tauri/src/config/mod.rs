use std::path::PathBuf;
use std::default::Default;
use std::{fs::File, fs};
use std::io::{prelude::*,Error,ErrorKind};
use serde::{Serialize, Deserialize};
use tauri::{Size,PhysicalSize, Position, PhysicalPosition};


pub mod config_plugin;

#[derive( Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct AppConfig {
    workspace: String,
    theme: String,
    x: i32,
    y: i32, 
    width: u32,
    height: u32,
}


impl Default for AppConfig{
    fn default() -> Self {
        AppConfig{
            workspace:String::from(""),
            theme:String::from("White"),
            x: 0i32,
            y: 0i32, 
            width: 800u32,
            height: 600u32,
        }  
    }
}

impl AppConfig{

    fn validate_path()-> Result<PathBuf,Error>{
        let local_dir = match tauri::api::path::local_data_dir(){
          Some(path) => path,
          None => return Err(Error::new(ErrorKind::Other, "Local Dir not Resolved"))
        };
        let config_dir = local_dir.join("sfm");
        if !config_dir.exists(){
          fs::create_dir(&config_dir)?;
        }
        let config_path = config_dir.join("config.join");
        if !config_path.exists(){
          let config_default:AppConfig = AppConfig::default();
          let config_str = serde_json::to_string(&config_default)?;
          println!("{}",config_str);
          File::create(&config_path)?.write_all(config_str.as_bytes())?;
        }
        Ok(config_path)
    }

    pub fn new() -> Self{
        let config_path = match AppConfig::validate_path(){
            Ok(path) => path,
            Err(error) => panic!("{}", error)
        };
        let config_file = File::open(config_path).expect("error while reading or parsing");
        serde_json::from_reader(config_file).expect("error parsing")
    }

    pub fn change_workspace(&mut self, workspace: String) {
        self.workspace = workspace.clone();
    }

    pub fn change_theme(&mut self, theme: String) {
        self.theme = theme.clone();
    }

    pub fn change_dimension(&mut self, width:u32, height:u32) {
        self.width = width;
        self.height = height;
    }

    pub fn change_position(&mut self, x:i32, y:i32) {
        self.x = x;
        self.y = y;
    }
    

    pub fn get_position(&self) -> Position{ 
        return  Position::Physical(
            PhysicalPosition{
                x: self.x,
                y: self.y
            }
        ); 
    }
    
    pub fn get_size(&self) -> Size { 
        return  Size::Physical(
            PhysicalSize{
                width: self.width,
                height: self.height
            }
        );
    }

    pub fn save_config(& self)-> Result<(),Error>{
        let config_path = match AppConfig::validate_path(){
            Ok(path) => path,
            Err(error) => panic!("{}", error)
        };
        let config_str = serde_json::to_string(&self)?;
        File::create(&config_path)?.write_all(config_str.as_bytes())?;
        Ok(())
    }
    
}