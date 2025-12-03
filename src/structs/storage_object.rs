use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageObject{
  #[serde(rename = "Key")]
  pub key: String,

  #[serde(rename = "Size")]
  pub size: u64
}