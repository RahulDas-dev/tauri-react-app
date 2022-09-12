mod workspace;
mod query;

pub use self::query::DbInstances; 

use std::ops::Deref;

use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use tauri::{plugin::Plugin, Invoke, Manager, Runtime};
use tauri::{api::path::{BaseDirectory, resolve_path},Env };
use tokio::sync::Mutex;

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

}

impl<R: Runtime> Plugin<R> for DatabasePlugin<R> {
  fn name(&self) -> &'static str {
    "database"
  }

  fn initialize(&mut self, app: &tauri::AppHandle<R>, _: serde_json::Value) -> tauri::plugin::Result<()> {

    let database_url = resolve_path(
      app.config().deref(),
      app.package_info().deref(),
      &Env::default(),
      "datastore.db",
      Some(BaseDirectory::App))
      .expect("Failed to resolve DB path")
      .to_str()
      .expect("error While Str Conv")
      .to_string();
    
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
}
