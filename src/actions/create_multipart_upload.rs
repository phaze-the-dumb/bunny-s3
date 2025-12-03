use std::sync::Arc;

use axum::{http::StatusCode, response::Response};

use crate::upload_manager::UploadManager;

pub async fn create_multipart_upload(
  bucket: String,
  path: String,
  upload_manager: Arc<UploadManager>
) -> Response{
  let upload_id = upload_manager.create_multipart(bucket.clone(), path.clone()).await;

  Response::builder()
    .status(StatusCode::OK)
    .body(format!("<InitiateMultipartUploadResult>
  <Bucket>{bucket}</Bucket>
  <Key>{path}</Key>
  <UploadId>{upload_id}</UploadId>
</InitiateMultipartUploadResult>").into())
    .unwrap()
}