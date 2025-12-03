use axum::{extract::Request, http::{Method, Response, StatusCode}};

use crate::{actions::{get_object::get_object, put_object::put_object}, auth::check_auth, util::error::{error, errors}};

pub async fn all(
  req: Request
) -> Response<String>{
  let auth = check_auth(&req);
  if let Err(err) = auth{
    dbg!(&err);
    errors(err.to_string(), StatusCode::INTERNAL_SERVER_ERROR)
  } else{
    match req.method().clone(){
      Method::GET => get_object(req.uri().path()),
      Method::PUT => put_object(req).await,
      _ => error("405 Not Found", StatusCode::METHOD_NOT_ALLOWED)
    }

  }
}