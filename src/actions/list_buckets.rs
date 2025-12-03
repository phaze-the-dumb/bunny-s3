use axum::http::{Response, StatusCode};

pub fn list_buckets() -> Response<String>{
  Response::builder()
    .status(StatusCode::OK)
    .header("Content-Type", "application/xml")
    .body("<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<ListAllMyBucketsResult>
  <Buckets>
    <Bucket>
      <Name>test</Name>
    </Bucket>
  </Buckets>
  <Prefix>/</Prefix>
</ListAllMyBucketsResult>".to_owned())
    .unwrap()
}