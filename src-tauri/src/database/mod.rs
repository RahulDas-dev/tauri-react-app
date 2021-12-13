use std::io::{Error,ErrorKind};

use sqlx::{Sqlite, SqlitePool, migrate::MigrateDatabase, sqlite::SqliteQueryResult};
use tauri::{plugin::Plugin, Runtime, Invoke, Manager, State};
use tokio::sync::Mutex;

pub type DbInstances = Mutex<SqlitePool>;

/* #[tauri::command]
async fn load_projects(db_state: State<'_, DbInstances>){
    let db_instanec = db_state.inner().lock().await;
    let qry = "SELECT * FROM project ORDER BY updated_on DESC";
    let rows = sqlx::query(&qry).fetch_all(&*db_instanec).await;
} */


#[tauri::command]
async fn add_project(db_state: State<'_, DbInstances>, image_dir: &str, out_dir: &str) ->Result<(u64, i64), String> {
    let db_instanec = db_state.inner().lock().await;
    let qry = "INSERT INTO project (img_directory,out_directory,status) VALUES ($1, $2, $3)";
    match sqlx::query(&qry)
    .bind(image_dir)
    .bind(out_dir)
    .bind("active").execute(&*db_instanec).await {
        Ok(result) => return Ok((result.rows_affected(), result.last_insert_rowid())),
        Err(e) => return Err(e.to_string())
    }
}

pub struct DatabasePlugin<R: Runtime> {
    invoke_handler: Box<dyn Fn(Invoke<R>) + Send + Sync>,
}

impl<R: Runtime> DatabasePlugin<R> {

    pub fn new()-> Self {
        Self {
            invoke_handler: Box::new(tauri::generate_handler![ add_project]),
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

    async fn cretea_schema(db_url:&str) -> Result<SqliteQueryResult, sqlx::Error> {
        let pool = SqlitePool::connect(&db_url).await?;
        let qry = 
        "PRAGMA foreign_keys = ON ;
        CREATE TABLE IF NOT EXISTS settings
            (
                settings_id             INTEGER PRIMARY KEY NOT NULL,
                description             TEXT                NOT NULL,
                created_on              DATETIME DEFAULT (datetime('now','localtime')),
                updated_on              DATETIME DEFAULT (datetime('now','localtime')),
                done                    BOOLEAN             NOT NULL DEFAULT 0
            );    
        CREATE TABLE IF NOT EXISTS project
            (
                project_id                   INTEGER PRIMARY KEY AUTOINCREMENT,
                created_on                   DATETIME DEFAULT (datetime('now','localtime')),
                updated_on                   DATETIME DEFAULT (datetime('now','localtime')),
                img_directory                TEXT NOT NULL,
                out_directory                TEXT NOT NULL,
                status                       TEXT NOT NULL,
                settings_id                  INTEGER  NOT NULL DEFAULT 1,
                FOREIGN KEY (settings_id)    REFERENCES settings (settings_id) ON UPDATE SET NULL ON DELETE SET NULL
            );";
        let result = sqlx::query(&qry).execute(&pool).await;   
        pool.close().await; 
        return result;
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

    fn extend_api(&mut self, message: Invoke<R>) {
        (self.invoke_handler)(message)
    }

    /* fn initialization_script(&self) -> Option<String> {None}

    fn on_page_load(&mut self, _: Window<R>, _: tauri::PageLoadPayload) {} */

}