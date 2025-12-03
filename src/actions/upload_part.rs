use std::sync::Arc;

use axum::{RequestExt, extract::Request, http::StatusCode, response::Response};
use futures_util::StreamExt;

use crate::upload_manager::UploadManager;

pub async fn upload_part(
  req: Request,
  part_number: usize,
  upload_id: String,
  upload_manager: Arc<UploadManager>
) -> Response{
  let mut body = req.into_limited_body().into_data_stream();
  let mut bytes = vec![];

  while let Some(part) = body.next().await{
    let part = part.unwrap();
    bytes.append(&mut part.to_vec());
  }

  if upload_manager.upload_multipart(part_number, upload_id, bytes).await.is_ok(){
    Response::builder()
      .status(StatusCode::OK)
      .body("".into())
      .unwrap()
  } else{
    Response::builder()
      .status(StatusCode::INTERNAL_SERVER_ERROR)
      .body("".into())
      .unwrap()
  }
}