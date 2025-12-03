use axum::http::{Response, StatusCode};

pub fn error( dat: &'static str, status: StatusCode ) -> Response<String>{
  Response::builder()
    .status(status)
    .body(dat.to_owned())
    .unwrap()
}

pub fn errors( dat: String, status: StatusCode ) -> Response<String>{
  Response::builder()
    .status(status)
    .body(dat)
    .unwrap()
}