use axum::{RequestExt, extract::Request, http::StatusCode, response::Response};
use futures_util::StreamExt;

pub async fn upload_part( req: Request, bucket: String, path: String, upload_id: String ) -> Response{
  let mut body = req.into_limited_body().into_data_stream();

  dbg!(&upload_id);

  let mut bytes = vec![];

  while let Some(part) = body.next().await{
    let part = part.unwrap();
    bytes.append(&mut part.to_vec());
  }

  dbg!(&bytes.len());

  Response::builder()
    .status(StatusCode::OK)
    .body("".into())
    .unwrap()
}