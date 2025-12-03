
use axum::{RequestExt, extract::Request, http::StatusCode, response::Response};
use reqwest::Body;

use crate::bunny;

pub async fn put_object( req: Request, bucket: String, path: String ) -> Response{
  let body = req.into_limited_body().into_data_stream();
  let stream = Body::wrap_stream(body);

  if bunny::upload_bunny_objects(bucket, path, stream).await.is_ok(){
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