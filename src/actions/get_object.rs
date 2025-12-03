use axum::{http::StatusCode, response::Response};

use crate::bunny::get_bunny_objects;

pub async fn get_object( bucket: String, path: String ) -> Response{
  Response::builder()
    .status(StatusCode::OK)
    .body(get_bunny_objects(bucket, path).await.unwrap().into())
    .unwrap()
}