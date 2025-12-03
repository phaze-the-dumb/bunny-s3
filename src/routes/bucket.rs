use std::collections::HashMap;

use axum::{extract::{Path, Query, Request}, http::StatusCode, response::Response};
use url_encor::Encoder;

use crate::{actions::list_objects::list_objects, auth::check_auth, util::error::errors};

pub async fn get(
  Path(bucket): Path<String>,
  Query(query): Query<HashMap<String, String>>,
  req: Request
) -> Response{
  let auth = check_auth(&req);
  if let Err(err) = auth{
    dbg!(&err);
    errors(err.to_string(), StatusCode::INTERNAL_SERVER_ERROR)
  } else{
    list_objects(bucket,
      query.get("delimiter").unwrap_or(&"/".to_owned()).clone().url_decode(),
      query.get("prefix").unwrap_or(&"".to_owned()).clone().url_decode()).await
  }
}