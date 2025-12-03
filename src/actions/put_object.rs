use std::{fs::{self, File}, io::Write};

use axum::{RequestExt, extract::Request, http::{Response, StatusCode}};
use futures_util::StreamExt;

pub async fn put_object<'a>( req: Request ) -> Response<String>{
  let path = req.uri().path().to_owned();
  let mut body = req.into_limited_body().into_data_stream();

  dbg!(&path);

  let mut bytes = vec![];

  while let Some(part) = body.next().await{
    let part = part.unwrap();
    bytes.append(&mut part.to_vec());
  }

  fs::write(format!("data/{path}"), bytes).unwrap();

  Response::builder()
    .status(StatusCode::OK)
    .body("".to_owned())
    .unwrap()
}