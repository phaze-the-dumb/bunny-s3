use std::sync::Arc;

use axum::{http::StatusCode, response::Response};

use crate::upload_manager::UploadManager;

pub async fn abort_multipart_upload(
  upload_id: String,
  upload_manager: Arc<UploadManager>
) -> Response{
  if upload_manager.abort_multipart(upload_id).await.is_ok(){
    Response::builder()
      .status(StatusCode::NO_CONTENT)
      .body("".into())
      .unwrap()
  } else{
    Response::builder()
      .status(StatusCode::INTERNAL_SERVER_ERROR)
      .body("".into())
      .unwrap()
  }
}