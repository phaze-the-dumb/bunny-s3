use std::{collections::HashMap, sync::Arc, thread, time::Duration};

use anyhow::bail;
use async_stream::stream;
use chrono::Utc;
use futures_util::Stream;
use nanoid::nanoid;
use reqwest::Body;
use tokio::sync::{Mutex, mpsc::{Receiver, Sender, channel}};

use crate::bunny;

#[derive(Clone)]
pub struct UploadManager{
  uploads: Arc<Mutex<HashMap<
    String,
    ( i64, Sender<UploadChunk>, Arc<Mutex<Receiver<u8>>> )>>>
}

pub enum UploadChunk{
  Chunk(Vec<u8>),
  End,
  Abort
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
      let ( time, _, _ ) = uploads.get(id).unwrap();

      if now - *time > 3600{
        uploads.remove(id).unwrap();
      }
    }
  }

  pub async fn create_multipart(&self, bucket: String, path: String) -> String{
    let mut uploads = self.uploads.lock().await;

    let now = Utc::now().timestamp();
    let id = nanoid!();

    let ( sender, recv ) = channel(128);
    let ( finished_sender, finished_recv ) = channel(1);

    tokio::spawn(async move {
      let aborted = Arc::new(Mutex::new(false));

      let stream = create_file_stream(recv, aborted.clone());
      let body = Body::wrap_stream(stream);

      if bunny::upload_bunny_objects(bucket.clone(), path.clone(), body).await.is_ok(){
        dbg!(&aborted);
        if *aborted.lock().await{
          let _ = bunny::delete_bunny_objects(bucket.clone(), path.clone()).await;
        }

        finished_sender.send(0u8).await.unwrap();
      } else{
        dbg!(&aborted);
        if *aborted.lock().await{
          let _ = bunny::delete_bunny_objects(bucket.clone(), path.clone()).await;
        }

        finished_sender.send(0u8).await.unwrap();
      }
    });

    uploads.insert(id.clone(), ( now, sender, Arc::new(Mutex::new(finished_recv)) ));
    id
  }

  pub async fn upload_multipart(&self, upload_id: String, upload_buf: Vec<u8>) -> anyhow::Result<()>{
    let mut uploads = self.uploads.lock().await;
    if let Some(( _, sender, _ )) = uploads.get_mut(&upload_id){
      sender.send(UploadChunk::Chunk(upload_buf)).await.unwrap();
      Ok(())
    } else{
      bail!("Invalid Upload ID");
    }
  }

  pub async fn complete_multipart(&self, upload_id: String) -> anyhow::Result<()>{
    let mut uploads = self.uploads.lock().await;
    if let Some(( _, sender, finished )) = uploads.remove(&upload_id){
      sender.send(UploadChunk::End).await.unwrap();
      finished.lock().await.recv().await.unwrap();

      Ok(())
    } else{
      bail!("Invalid Upload ID");
    }
  }

  pub async fn abort_multipart(&self, upload_id: String) -> anyhow::Result<()>{
    let mut uploads = self.uploads.lock().await;
    if let Some(( _, sender, _ )) = uploads.remove(&upload_id){
      sender.send(UploadChunk::Abort).await.unwrap();
    }

    Ok(()) // Return Ok if it aborted or not, if the upload isn't running, then it's already aborted
  }
}

fn create_file_stream( mut recv: Receiver<UploadChunk>, aborted: Arc<Mutex<bool>> ) -> impl Stream<Item = anyhow::Result<Vec<u8>>> {
  stream! {
    while let Some(chunk) = recv.recv().await{
      match chunk{
        UploadChunk::Chunk(data) => yield Ok(data),
        UploadChunk::End => break,
        UploadChunk::Abort => {
          *aborted.lock().await = true;
          break
        }
      }
    }
  }
}