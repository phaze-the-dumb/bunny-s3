use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Prefix{
  #[serde(rename = "Prefix")]
  pub prefix: String
}