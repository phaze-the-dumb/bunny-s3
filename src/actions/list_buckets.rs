use axum::{http::StatusCode, response::Response};

pub fn list_buckets() -> Response{
  Response::builder()
    .status(StatusCode::OK)
    .header("Content-Type", "application/xml")
    .body("<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<ListAllMyBucketsResult>
  <Buckets>
    <Bucket>
      <Name>phazecdn</Name>
    </Bucket>
  </Buckets>
  <Prefix>/</Prefix>
</ListAllMyBucketsResult>".into())
    .unwrap()
}