use std::{collections::HashMap, sync::Arc, thread, time::Duration};

use anyhow::bail;
use chrono::Utc;
use nanoid::nanoid;
use tokio::sync::Mutex;

use crate::bunny;

#[derive(Clone)]
pub struct UploadManager{
  uploads: Arc<Mutex<HashMap<String, ( i64, Vec<( usize, Vec<u8> )>, String, String )>>>
}

impl UploadManager{
  pub fn new() -> Self{
    let manager = Self {
      uploads: Arc::new(Mutex::new(HashMap::new()))
    };

    let manager_1 = manager.clone();

    tokio::spawn(async move {
      thread::sleep(Duration::from_secs(60));
      manager_1.clean_old_downloads().await;
    });

    manager
  }

  pub async fn clean_old_downloads(&self){
    let mut uploads = self.uploads.lock().await;
    let now = Utc::now().timestamp();

    for id in uploads.clone().keys(){
      let ( time, _, _, _ ) = uploads.get(id).unwrap();

      if now - *time > 3600{
        uploads.remove(id).unwrap();
      }
    }
  }

  pub async fn create_multipart(&self, bucket: String, path: String) -> String{
    let mut uploads = self.uploads.lock().await;

    let now = Utc::now().timestamp();
    let id = nanoid!();

    uploads.insert(id.clone(), ( now, vec![], bucket, path ));
    id
  }

  pub async fn upload_multipart(&self, part_number: usize, upload_id: String, upload_buf: Vec<u8>) -> anyhow::Result<()>{
    let mut uploads = self.uploads.lock().await;
    if let Some(( _, buf, _, _ )) = uploads.get_mut(&upload_id){
      buf.push(( part_number, upload_buf ));
      Ok(())
    } else{
      bail!("Invalid Upload ID");
    }
  }

  pub async fn complete_multipart(&self, upload_id: String) -> anyhow::Result<()>{ // TODO: Please find a way to actually make this streamed
    let mut uploads = self.uploads.lock().await;
    if let Some(( _, buf, bucket, key )) = uploads.get_mut(&upload_id){
      buf.sort_by(|a, b| a.0.cmp(&b.0));

      let mut bytes = vec![];
      for ( _, chunk ) in buf{ bytes.append(chunk); }

      if bunny::upload_bunny_objects_unstreamed(bucket.clone(), key.clone(), bytes).await.is_ok(){
        Ok(())
      } else{
        bail!("Failed Upload");
      }
    } else{
      bail!("Invalid Upload ID");
    }
  }
}