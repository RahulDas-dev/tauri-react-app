use serde::{Deserialize, Serialize};
use std::default::Default;


#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct WorkSpace {
  pub project_id: i64,
  pub created_on: String,
  pub updated_on: String,
  pub input_directory: String,
  pub output_directory: String,
  pub status: String,
  pub settings_id: i64,
}
/* 
pub struct WorkSpaceRow {
  project_id: i64,
  workspaceinfo: Json<WorkSpace>,
} */


impl Default for WorkSpace {
  fn default() -> Self {
    WorkSpace {
      project_id: -1i64,
      created_on: String::from(""),
      updated_on: String::from(""),
      input_directory: String::from(""),
      output_directory: String::from(""),
      status: String::from("Active"),
      settings_id: 1i64,
    }
  }
}
