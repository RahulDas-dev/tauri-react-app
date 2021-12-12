use std::io::{prelude::*,Error,ErrorKind};
use sqlx::{Sqlite, SqlitePool, migrate::MigrateDatabase, query};
use tauri::{plugin::Plugin, Runtime, Invoke, Manager};
use tokio::sync::Mutex;

type DbInstances = Mutex<SqlitePool>;

#[tauri::command]
async fn load(){
    println!("Loading ...");
}

#[tauri::command]
async fn select(){
    println!("select ...");
}

pub struct DatabasePlugin<R: Runtime> {
    invoke_handler: Box<dyn Fn(Invoke<R>) + Send + Sync>,
}

impl<R: Runtime> DatabasePlugin<R> {

    pub fn new()-> Self {
        Self {
            invoke_handler: Box::new(tauri::generate_handler![load, select]),
        }
    }

    fn resolve_database_url() -> Result<String,Error>{
        let database_path = match tauri::api::path::local_data_dir(){
            Some(path) => match path.join("sfm").join("datastore.db").to_str(){
                Some( path_in_str) => path_in_str.to_owned(),
                None => return Err(Error::new(ErrorKind::Other, "Database Path not Resolved"))
            }
            None => return Err(Error::new(ErrorKind::Other, "Local Directory not Resolved"))
        };
        return Ok(format!("sqlite://{}", database_path));
    }

    async fn cretea_schema(database_url: &str)-> Result<bool, sqlx::Error>{
        let pool = SqlitePool::connect(database_url).await?;
        let qry1 ="CREATE TABLE IF NOT EXISTS project
            (
            id          INTEGER PRIMARY KEY NOT NULL,
            description TEXT                NOT NULL,
            done        BOOLEAN             NOT NULL DEFAULT 0
            )";
        let qry2 ="CREATE TABLE IF NOT EXISTS settings
            (
            id          INTEGER PRIMARY KEY NOT NULL,
            description TEXT                NOT NULL,
            done        BOOLEAN             NOT NULL DEFAULT 0
            )";    
        let result = query(&qry1).execute(&pool).await?;
        println!("{:?}", result);
        let result = query(&qry2).execute(&pool).await?;
        println!("{:?}", result);
        pool.close().await;
        Ok(true)
    }
}

impl<R: Runtime> Plugin<R> for DatabasePlugin<R> {

    fn name(&self) -> &'static str {"databse"}

    fn initialize(&mut self, app: &tauri::AppHandle<R>, _: serde_json::Value) -> tauri::plugin::Result<()> {
        let database_url =Self::resolve_database_url()?;
        tauri::async_runtime::block_on(async move {
            if !Sqlite::database_exists(&database_url).await.unwrap_or(false) {
                Sqlite::create_database(&database_url).await.unwrap();
                Self::cretea_schema(&database_url).await.unwrap();
            }
            let instances = SqlitePool::connect(&database_url).await.unwrap();
            let db_instance :DbInstances = Mutex::new(instances);
            app.manage(db_instance);
            Ok(())
        })
    }

}