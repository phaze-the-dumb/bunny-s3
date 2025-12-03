use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BunnyStorageObject{
  #[serde(rename = "ObjectName")]
  pub name: String,

  #[serde(rename = "IsDirectory")]
  pub is_directory: bool,

  #[serde(rename = "Length")]
  pub size: u64
}