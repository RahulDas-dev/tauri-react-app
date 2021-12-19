mod workspace;

use std::io::{Error, ErrorKind};

use serde_json::Value as JsonValue;
use sqlx::{migrate::MigrateDatabase, sqlite::SqliteQueryResult, Column, Row, Sqlite, SqlitePool};
use tauri::{plugin::Plugin, Invoke, Manager, Runtime, State};
use tokio::sync::Mutex;

use workspace::WorkSpace;

pub type DbInstances = Mutex<SqlitePool>;

#[tauri::command]
async fn add_workspace(
  db_state: State<'_, DbInstances>,
  input_dir: &str,
  output_dir: &str,
) -> Result<(u64, i64), String> {
  let db_instanec = db_state.inner().lock().await;
  let qry = "INSERT INTO workspace (input_directory,output_directory,status) VALUES ($1, $2, $3)";
  match sqlx::query(&qry)
    .bind(input_dir)
    .bind(output_dir)
    .bind("active")
    .execute(&*db_instanec)
    .await
  {
    Ok(result) => return Ok((result.rows_affected(), result.last_insert_rowid())),
    Err(e) => return Err(e.to_string()),
  }
}

#[tauri::command]
async fn get_work_history(db_state: State<'_, DbInstances>) -> Result<Vec<JsonValue>, String> {
  let db_instanec = db_state.inner().lock().await;
  let qry = "SELECT * FROM workspace ORDER BY updated_on DESC";
  let rows = sqlx::query(&qry).fetch_all(&*db_instanec).await;
  match rows {
    Ok(results) => {
      let mut result_list: Vec<JsonValue> = Vec::new();
      for row_data in results.into_iter() {
        let mut workspace = WorkSpace::default();
        for (i, column) in row_data.columns().iter().enumerate() {
          match column.name() {
            "project_id" => {
              let data = row_data.get::<i64, usize>(i);
              workspace.project_id = data
            }
            "created_on" => {
              let data = row_data.get::<String, usize>(i);
              workspace.created_on = data.to_string()
            }
            "updated_on" => {
              let data = row_data.get::<String, usize>(i);
              workspace.updated_on = data.to_string()
            }
            "input_directory" => {
              let data = row_data.get::<String, usize>(i);
              workspace.input_directory = data.to_string()
            }
            "output_directory" => {
              let data = row_data.get::<String, usize>(i);
              workspace.output_directory = data.to_string()
            }
            "status" => {
              let data = row_data.get::<String, usize>(i);
              workspace.status = data.to_string()
            }
            "settings_id" => {
              let data = row_data.get::<i64, usize>(i);
              workspace.settings_id = data
            }
            _ => continue,
          };
        }
        match serde_json::to_value(workspace) {
          Ok(workspace_json) => result_list.push(workspace_json),
          Err(_) => {}
        }
      }
      Ok(result_list)
    }
    Err(e) => return Err(e.to_string()),
  }
}

pub struct DatabasePlugin<R: Runtime> {
  invoke_handler: Box<dyn Fn(Invoke<R>) + Send + Sync>,
}

impl<R: Runtime> DatabasePlugin<R> {
  pub fn new() -> Self {
    Self {
      invoke_handler: Box::new(tauri::generate_handler![add_workspace, get_work_history]),
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

  async fn cretea_schema(db_url: &str) -> Result<SqliteQueryResult, sqlx::Error> {
    let pool = SqlitePool::connect(&db_url).await?;
    let qry = "PRAGMA foreign_keys = ON ;
        CREATE TABLE IF NOT EXISTS settings
            (
                settings_id             INTEGER PRIMARY KEY NOT NULL,
                description             TEXT                NOT NULL,
                created_on              DATETIME DEFAULT (datetime('now','localtime')),
                updated_on              DATETIME DEFAULT (datetime('now','localtime')),
                feature_type            TEXT     DEFAULT 'SIFT' NOT NULL,
                max_keypoint            INTEGER  DEFAULT 5000 NOT NULL 
            );    
        CREATE TABLE IF NOT EXISTS workspace
            (
                project_id                   INTEGER PRIMARY KEY AUTOINCREMENT,
                created_on                   DATETIME DEFAULT (datetime('now','localtime')),
                updated_on                   DATETIME DEFAULT (datetime('now','localtime')),
                input_directory              TEXT NOT NULL,
                output_directory             TEXT NOT NULL,
                status                       TEXT NOT NULL,
                settings_id                  INTEGER  NOT NULL DEFAULT 1,
                FOREIGN KEY (settings_id)    REFERENCES settings (settings_id) ON UPDATE SET NULL ON DELETE SET NULL
            );
        INSERT INTO settings (description) VALUES ('DEFAULT SETTINGS');";
    let result = sqlx::query(&qry).execute(&pool).await;
    pool.close().await;
    return result;
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
        Self::cretea_schema(&database_url).await.unwrap();
      }
      let instances = SqlitePool::connect(&database_url).await.unwrap();
      let db_instance: DbInstances = Mutex::new(instances);
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
