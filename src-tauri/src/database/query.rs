
use serde_json::Value as JsonValue;
use sqlx::{Column, Row, SqlitePool, sqlite::SqliteQueryResult};
use tauri::State;
use tokio::sync::Mutex;
use super::workspace::WorkSpace;
pub type DbInstances = Mutex<SqlitePool>;


pub async fn cretea_schema(db_url: &str) -> Result<SqliteQueryResult, sqlx::Error> {
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

#[tauri::command]
pub async fn add_workspace(
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
pub async fn get_work_history(db_state: State<'_, DbInstances>) -> Result<Vec<JsonValue>, String> {
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
