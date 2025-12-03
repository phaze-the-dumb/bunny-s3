use axum::{extract::Request, http::{Response, StatusCode}};

use crate::{actions::list_buckets::list_buckets, auth::check_auth, util::error::errors};

pub async fn get( req: Request ) -> Response<String>{
  let auth = check_auth(&req);
  if let Err(err) = auth{
    dbg!(&err);
    errors(err.to_string(), StatusCode::INTERNAL_SERVER_ERROR)
  } else{
    list_buckets()
  }
}