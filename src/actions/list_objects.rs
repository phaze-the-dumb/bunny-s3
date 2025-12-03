use axum::{http::StatusCode, response::Response};
use url_encor::Encoder;

use crate::{bunny, structs::{list_bucket_result::ListBucketResult, prefixes::Prefix, storage_object::StorageObject}};

pub async fn list_objects( bucket: String, delimiter: String, prefix: String ) -> Response{
  let prefix = prefix.url_decode();

  if let Ok(dir_contents) = bunny::list_bunny_objects(bucket.clone(), prefix.clone()).await{
    let mut files = vec![];
    let mut folders = vec![];

    for file in dir_contents{
      if file.is_directory{
        folders.push(Prefix{ prefix: file.name });
      } else{
        files.push(StorageObject { key: file.name });
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
      .body(serde_xml_rs::to_string(&doc).unwrap().into())
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
  </ListBucketResult>").into())
      .unwrap()
  }
}