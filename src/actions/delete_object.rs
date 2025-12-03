use axum::{http::StatusCode, response::Response};

use crate::bunny;

pub async fn delete_object( bucket: String, path: String ) -> Response{
  if bunny::delete_bunny_objects(bucket, path).await.is_ok(){
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