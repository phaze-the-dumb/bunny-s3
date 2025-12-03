use serde::{Deserialize, Serialize};

use crate::structs::{prefixes::Prefix, storage_object::StorageObject};

#[derive(Serialize, Deserialize, Debug)]
pub struct ListBucketResult{
  #[serde(rename = "Name")]
  pub name: String,

  #[serde(rename = "Contents")]
  pub contents: Vec<StorageObject>,

  #[serde(rename = "CommonPrefixes")]
  pub common_prefixes: Vec<Prefix>
}