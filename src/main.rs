use axum::{Router, extract::Request, http::HeaderMap, response::{IntoResponse, Response}, routing::get};
use hmac::{Hmac, Mac, digest::FixedOutput};
use sha2::{Digest, Sha256};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::level_filters::LevelFilter;

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug)]
struct AuthorizationHeader{
  algorithm: String,
  key_id: String,
  date: String,
  region: String,
  service: String,
  signed_headers: Vec<String>,
  signature: String
}

fn parse_auth_header<'a>(auth: &'a str) -> AuthorizationHeader{
  let mut auth = auth.split(' ');
  let algorithm = auth.nth(0).unwrap();

  let auth = auth.collect::<Vec<_>>().join(" ");
  let auth = auth.split(",").collect::<Vec<_>>();

  let mut header = AuthorizationHeader {
    algorithm: algorithm.to_owned(),
    key_id: "None".to_owned(),
    date: "None".to_owned(),
    region: "None".to_owned(),
    service: "None".to_owned(),
    signed_headers: vec![],
    signature: "None".to_owned()
  };

  for kv in auth{
    let kv = kv.split("=").collect::<Vec<_>>();
    let key = *kv.first().unwrap();
    let value = *kv.last().unwrap();

    match key{
      "Credential" => {
        let cred = value.split('/').collect::<Vec<_>>();

        header.key_id = cred[0].to_owned();
        header.date = cred[1].to_owned();
        header.region = cred[2].to_owned();
        header.service = cred[3].to_owned();
      }
      "SignedHeaders" => {
        header.signed_headers = value.split(";").map(|x| x.to_owned().to_lowercase()).collect();
      }
      "Signature" => {
        header.signature = value.to_owned();
      }
      _ => {}
    }
  }

  header
}

async fn test( req: Request ) -> impl IntoResponse{
  let headers = req.headers();
  let auth = parse_auth_header(headers.get("authorization").unwrap().to_str().unwrap());

  let mut date_key = HmacSha256::new_from_slice(format!("AWS4{}", "test2").as_bytes()).unwrap();
  date_key.update(auth.date.as_bytes());
  let date_key = date_key.finalize_fixed();

  let mut date_region_key = HmacSha256::new_from_slice(&date_key).unwrap();
  date_region_key.update(auth.region.as_bytes());
  let date_region_key = date_region_key.finalize_fixed();

  let mut date_region_service_key = HmacSha256::new_from_slice(&date_region_key).unwrap();
  date_region_service_key.update(auth.service.as_bytes());
  let date_region_service_key = date_region_service_key.finalize_fixed();

  let mut signing_key = HmacSha256::new_from_slice(&date_region_service_key).unwrap();
  signing_key.update("aws4_request".as_bytes());
  let signing_key = signing_key.finalize_fixed();

  let mut canonical_headers = vec![];

  for (key, value) in headers{
    let key = key.to_string().to_lowercase();

    if auth.signed_headers.contains(&key){
      canonical_headers.push(format!("{}:{}\n", key, value.to_str().unwrap()));
    }
  }

  canonical_headers.sort();

  let canonical_request = format!(
    "{}\n{}\n{}\n{}\n{}\n{}",
    req.method().as_str(),
    req.uri().path(),
    req.uri().query().unwrap_or(""),
    canonical_headers.join(""),
    auth.signed_headers.join(";"),
    headers.get("x-amz-content-sha256").unwrap().to_str().unwrap()
  );

  let req_hash = hex::encode(Sha256::digest(canonical_request.as_bytes()));

  let string_to_sign = format!(
    "AWS4-HMAC-SHA256\n{}\n{}/{}/s3/aws4_request\n{}",
    headers.get("x-amz-date").unwrap().to_str().unwrap(),
    auth.date, auth.region,
    req_hash
  );

  let mut signature = HmacSha256::new_from_slice(&signing_key).unwrap();
  signature.update(string_to_sign.as_bytes());
  let signature = signature.finalize_fixed();

  let signature = hex::encode(signature);
  dbg!(auth.signature);
  dbg!(signature);

  Response::builder()
    .status(200)
    .header("Content-Type", "application/xml")
    .body("<?xml version=\"1.0\" encoding=\"UTF-8\"?>".to_owned())
    .unwrap()
}

#[tokio::main]
async fn main() -> anyhow::Result<()>{
  tracing_subscriber::fmt().with_max_level(LevelFilter::DEBUG).init();

  let app = Router::new()
    .fallback(test)
    .layer(TraceLayer::new_for_http());

  let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
  axum::serve(listener, app).await.unwrap();

  Ok(())
}