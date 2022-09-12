mod workspace;
mod query;

pub use self::query::DbInstances; 

use std::io::{Error, ErrorKind};

use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use tauri::{plugin::Plugin, Invoke, Manager, Runtime};
use tokio::sync::Mutex;

//use query::DbInstances;

pub struct DatabasePlugin<R: Runtime> {
  invoke_handler: Box<dyn Fn(Invoke<R>) + Send + Sync>,
}

impl<R: Runtime> DatabasePlugin<R> {
  pub fn new() -> Self {
    println!("DB Plugin init");
    Self {
      invoke_handler: Box::new(tauri::generate_handler![query::add_workspace, 
                                                        query::get_work_history]),
    }
  }

  fn resolve_database_url() -> Result<String, Error> {
    let database_path = match tauri::api::path::local_data_dir() {
      Some(path) => match path.join("sfm").join("datastore.db").to_str() {
        Some(path_in_str) => path_in_str.to_owned(),
        None => return Err(Error::new(ErrorKind::Other, "Database Path not Resolved")),
      },
      None => return Err(Error::new(ErrorKind::Other, "Local Directory not Resolved")),
    };
    return Ok(format!("sqlite://{}", database_path));
  }

}

impl<R: Runtime> Plugin<R> for DatabasePlugin<R> {
  fn name(&self) -> &'static str {
    "database"
  }

  fn initialize(&mut self, app: &tauri::AppHandle<R>, _: serde_json::Value) -> tauri::plugin::Result<()> {
    let database_url = Self::resolve_database_url()?;
    tauri::async_runtime::block_on(async move {
      if !Sqlite::database_exists(&database_url).await.unwrap_or(false) {
        Sqlite::create_database(&database_url).await.unwrap();
        query::cretea_schema(&database_url).await.unwrap();
      }
      let instances = SqlitePool::connect(&database_url).await.unwrap();
      let db_instance: query::DbInstances = Mutex::new(instances);
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
