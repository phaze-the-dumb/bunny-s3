use std::collections::HashMap;

use axum::{extract::{Query, Request}, http::{Method, StatusCode}, response::Response};

use crate::{actions::{create_multipart_upload::create_multipart_upload, delete_object::delete_object, get_object::get_object, put_object::put_object, upload_part::upload_part}, auth::check_auth, util::error::{error, errors}};

#[axum::debug_handler]
pub async fn all(
  Query(query): Query<HashMap<String, String>>,
  req: Request
) -> Response{
  let auth = check_auth(&req);
  if let Err(err) = auth{
    dbg!(&err);
    errors(err.to_string(), StatusCode::INTERNAL_SERVER_ERROR)
  } else{
    let mut uri_path = req.uri().path().split("/");

    let bucket = uri_path.nth(0).unwrap().to_owned();
    let path = uri_path.collect::<Vec<_>>().join("/");

    match req.method().clone(){
      Method::GET => get_object(bucket, path).await,
      Method::DELETE => delete_object(bucket, path).await,
      Method::POST => create_multipart_upload(req, bucket, path).await,
      Method::PUT => {
        if let Some(upload_id) = query.get("uploadId"){
          upload_part(req, bucket, path, upload_id.clone()).await
        } else{
          put_object(req, bucket, path).await
        }
      }
      _ => error("405 Not Found", StatusCode::METHOD_NOT_ALLOWED)
    }

  }
}