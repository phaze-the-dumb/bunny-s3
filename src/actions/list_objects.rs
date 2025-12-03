use std::fs;

use axum::http::{Response, StatusCode};
use url_encor::Encoder;

use crate::structs::{list_bucket_result::ListBucketResult, prefixes::Prefix, storage_object::StorageObject};

pub fn list_objects( bucket: String, delimiter: String, prefix: String ) -> Response<String>{
  let prefix = prefix.url_decode();

  if let Ok(dir_contents) = fs::read_dir(format!("data/{bucket}/{prefix}")){
    let mut files = vec![];
    let mut folders = vec![];

    for file in dir_contents{
      let file = file.unwrap();

      if file.metadata().unwrap().is_file(){
        files.push(StorageObject { key: file.file_name().to_str().unwrap().to_owned() });
      } else{
        folders.push(Prefix{ prefix: file.file_name().to_str().unwrap().to_owned() });
      }
    }

    let doc = ListBucketResult {
      name: "help".to_owned(),
      contents: files,
      common_prefixes: folders
    };

    // dbg!(serde_xml_rs::to_string(&doc).unwrap());

    Response::builder()
      .status(StatusCode::OK)
      .header("Content-Type", "application/xml")
      .body(serde_xml_rs::to_string(&doc).unwrap())
      .unwrap()
  } else{
    Response::builder()
      .status(StatusCode::OK)
      .header("Content-Type", "application/xml")
      .body(format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>
  <ListBucketResult>
    <Prefix>{prefix}</Prefix>
    <Delimiter>{delimiter}</Delimiter>
    <Name>{bucket}</Name>
  </ListBucketResult>"))
      .unwrap()
  }
}