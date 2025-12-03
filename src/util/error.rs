use axum::{http::StatusCode, response::Response};

pub fn error( dat: &'static str, status: StatusCode ) -> Response{
  Response::builder()
    .status(status)
    .body(dat.into())
    .unwrap()
}

pub fn errors( dat: String, status: StatusCode ) -> Response{
  Response::builder()
    .status(status)
    .body(dat.into())
    .unwrap()
}