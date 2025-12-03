use std::env;

use anyhow::bail;
use axum::body::Bytes;
use reqwest::{Body, Client};

use crate::bunny::structs::BunnyStorageObject;

mod structs;

pub async fn list_bunny_objects( bucket: String, path: String ) -> anyhow::Result<Vec<BunnyStorageObject>>{
  let endpoint = env::var("ENDPOINT")?;
  let api_token = env::var("API_TOKEN")?;

  let url = format!("https://{endpoint}/{bucket}/{path}/");

  let res = Client::default().get(url)
    .header("Accept", "application/json")
    .header("AccessKey", api_token)
    .send().await?;

  if res.status() != 200{ bail!("Request failed.") }

  let json = res.text().await?;
  let json: Vec<BunnyStorageObject> = serde_json::from_str(&json)?;

  Ok(json)
}

pub async fn get_bunny_object( bucket: String, key: String ) -> anyhow::Result<Bytes>{
  let endpoint = env::var("ENDPOINT")?;
  let api_token = env::var("API_TOKEN")?;

  let url = format!("https://{endpoint}/{bucket}/{key}");

  let res = Client::default().get(url)
    .header("Accept", "application/octet-stream")
    .header("AccessKey", api_token)
    .send().await?;

  if res.status() != 200{ bail!("Request failed.") }
  let bytes = res.bytes().await.unwrap();

  Ok(bytes)
}

pub async fn delete_bunny_objects( bucket: String, key: String ) -> anyhow::Result<()>{
  let endpoint = env::var("ENDPOINT")?;
  let api_token = env::var("API_TOKEN")?;

  let url = format!("https://{endpoint}/{bucket}/{key}");

  let res = Client::default().delete(url)
    .header("Accept", "application/json")
    .header("AccessKey", api_token)
    .send().await?;

  if res.status() != 200{ bail!("Request failed.") }
  Ok(())
}

pub async fn upload_bunny_objects( bucket: String, key: String, stream: Body ) -> anyhow::Result<()>{
  let endpoint = env::var("ENDPOINT")?;
  let api_token = env::var("API_TOKEN")?;

  let url = format!("https://{endpoint}/{bucket}/{key}");

  let res = Client::default().put(url)
    .header("Content-Type", "application/octet-stream")
    .header("AccessKey", api_token)
    .body(stream)
    .send().await?;

  if res.status() != 201{ bail!("Request failed.") }
  Ok(())
}

pub async fn upload_bunny_objects_unstreamed( bucket: String, key: String, body: Vec<u8> ) -> anyhow::Result<()>{
  let endpoint = env::var("ENDPOINT")?;
  let api_token = env::var("API_TOKEN")?;

  let url = format!("https://{endpoint}/{bucket}/{key}");

  let res = Client::default().put(url)
    .header("Content-Type", "application/octet-stream")
    .header("AccessKey", api_token)
    .body(body)
    .send().await?;

  if res.status() != 201{ bail!("Request failed.") }
  Ok(())
}