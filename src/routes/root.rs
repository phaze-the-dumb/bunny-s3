use std::{collections::HashMap, env};

use axum::{extract::{Query, Request}, http::StatusCode, response::Response};
use url_encor::Encoder;

use crate::{actions::{list_buckets::list_buckets, list_objects::list_objects}, auth::check_auth, util::error::{error, errors}};

pub async fn get(
  Query(query): Query<HashMap<String, String>>,
  req: Request
) -> Response{
  let auth = check_auth(&req);

  if let Err(err) = auth{
    dbg!(&err);
    errors(err.to_string(), StatusCode::INTERNAL_SERVER_ERROR)
  } else{
    if let Some(host_header) = req.headers().get("host"){
      let host_header = host_header.to_str().unwrap().to_owned();
      let host_header = host_header.split(":").nth(0).unwrap(); // Remove port from host header

      let host = env::var("HOSTNAME").unwrap_or("localhost".to_owned());

      if host_header == host{
        list_buckets()
      } else{
        let bucket = host_header.replace(&format!(".{host}"), "");

        list_objects(bucket,
          query.get("delimiter").unwrap_or(&"/".to_owned()).clone().url_decode(),
          query.get("prefix").unwrap_or(&"".to_owned()).clone().url_decode()).await
      }
    } else{
      error("Uhh, this shouldn't be possible?", StatusCode::IM_A_TEAPOT)
    }
  }
}