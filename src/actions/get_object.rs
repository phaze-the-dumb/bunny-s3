use std::fs;

use axum::http::{Response, StatusCode};

pub fn get_object<'a>( path: &'a str ) -> Response<String>{
  Response::builder()
    .status(StatusCode::OK)
    .body(fs::read_to_string(format!("data/{path}")).unwrap())
    .unwrap()
}