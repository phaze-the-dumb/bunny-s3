use axum::{RequestExt, extract::Request, http::StatusCode, response::Response};
use futures_util::StreamExt;

pub async fn create_multipart_upload( req: Request, bucket: String, path: String ) -> Response{
  let path = req.uri().path().to_owned();
  let mut body = req.into_limited_body().into_data_stream();

  dbg!(&path);

  let mut bytes = vec![];

  while let Some(part) = body.next().await{
    let part = part.unwrap();
    bytes.append(&mut part.to_vec());
  }

  let cmd = str::from_utf8(&bytes).unwrap();

  if cmd == ""{
    Response::builder()
      .status(StatusCode::OK)
      .body("<InitiateMultipartUploadResult>
    <Bucket>test</Bucket>
    <Key>string</Key>
    <UploadId>string</UploadId>
  </InitiateMultipartUploadResult>".into())
      .unwrap()
  } else{
    Response::builder()
      .status(StatusCode::OK)
      .body("<CompleteMultipartUploadResult>
</CompleteMultipartUploadResult>".into())
      .unwrap()
  }


}