use std::env;

use axum::{http::StatusCode, response::Response};

pub fn list_buckets() -> Response{
  let bucket = env::var("BUCKET_NAME").unwrap();

  Response::builder()
    .status(StatusCode::OK)
    .header("Content-Type", "application/xml")
    .body(format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<ListAllMyBucketsResult>
  <Buckets>
    <Bucket>
      <Name>{bucket}</Name>
    </Bucket>
  </Buckets>
  <Prefix>/</Prefix>
</ListAllMyBucketsResult>").into())
    .unwrap()
}