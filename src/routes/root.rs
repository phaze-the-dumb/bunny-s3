use axum::{extract::Request, http::StatusCode, response::Response};

use crate::{actions::list_buckets::list_buckets, auth::check_auth, util::error::errors};

pub async fn get( req: Request ) -> Response{
  // TODO: Detect if client is trying to use virtual-host style and respond with object list instead of bucket list
  let auth = check_auth(&req);
  if let Err(err) = auth{
    dbg!(&err);
    errors(err.to_string(), StatusCode::INTERNAL_SERVER_ERROR)
  } else{
    list_buckets()
  }
}