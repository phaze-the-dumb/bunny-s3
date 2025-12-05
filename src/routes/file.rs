use std::{collections::HashMap, env, sync::Arc};

use axum::{Extension, extract::{Query, Request}, http::{Method, StatusCode}, response::Response};

use crate::{actions::{abort_multipart_upload::abort_multipart_upload, complete_multipart_upload::complete_multipart_upload, create_multipart_upload::create_multipart_upload, delete_object::delete_object, get_object::get_object, put_object::put_object, upload_part::upload_part}, auth::check_auth, upload_manager::UploadManager, util::error::{error, errors}};

#[axum::debug_handler]
pub async fn all(
  Query(query): Query<HashMap<String, String>>,
  Extension(upload_manager): Extension<Arc<UploadManager>>,
  req: Request
) -> Response{
  let auth = check_auth(&req);
  if let Err(err) = auth{
    dbg!(&err);
    errors(err.to_string(), StatusCode::INTERNAL_SERVER_ERROR)
  } else{
    let mut uri_path = req.uri().path().split("/");

    let bucket = if let Some(host_header) = req.headers().get("host"){
      let host_header = host_header.to_str().unwrap().to_owned();
      let host_header = host_header.split(":").nth(0).unwrap(); // Remove port from host header

      let host = env::var("HOSTNAME").unwrap_or("localhost".to_owned());

      if host_header == host{
        uri_path.nth(0).unwrap().to_owned()
      } else{
        host_header.replace(&format!(".{host}"), "")
      }
    } else{
      "".to_owned()
    };

    let path = uri_path.collect::<Vec<_>>().join("/");

    match req.method().clone(){
      Method::GET => get_object(bucket, path).await,
      Method::DELETE => {
        if let Some(upload_id) = query.get("uploadId"){
          abort_multipart_upload(upload_id.clone(), upload_manager).await
        } else{
          delete_object(bucket, path).await
        }
      }
      Method::POST => {
        if let Some(upload_id) = query.get("uploadId"){
          complete_multipart_upload(upload_id.clone(), upload_manager).await
        } else{
          create_multipart_upload(bucket, path, upload_manager).await
        }
      },
      Method::PUT => {
        if let Some(upload_id) = query.get("uploadId"){
          upload_part(req, upload_id.clone(), upload_manager).await
        } else{
          put_object(req, bucket, path).await
        }
      }
      _ => error("405 Not Found", StatusCode::METHOD_NOT_ALLOWED)
    }

  }
}