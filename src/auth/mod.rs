use std::env;

use anyhow::bail;
use axum::extract::Request;
use hmac::{Hmac, Mac, digest::FixedOutput};
use sha2::{Digest, Sha256};

type HmacSha256 = Hmac<Sha256>;

const CLIENT_ALGORITHM: &'static str = "AWS4-HMAC-SHA256";

pub fn check_auth( req: &Request ) -> anyhow::Result<()>{
  let client_key_id = env::var("S3_CLIENT_KEY_ID").unwrap();
  let client_secret = env::var("S3_CLIENT_SECRET").unwrap();

  let headers = req.headers();

  let Some(auth) = headers.get("Authorization") else { bail!("No authorization header") };
  let auth = parse_auth_header(auth.to_str()?)?;

  if auth.algorithm != CLIENT_ALGORITHM{ bail!("Unsupported Algorithm") }
  if auth.key_id != client_key_id{ bail!("Invalid KEY ID") }

  let Some(x_amz_date) = headers.get("x-amz-date") else { bail!("No X-AMZ-DATE header.") };
  let x_amz_date = x_amz_date.to_str()?;

  let x_amz_content_sha256 = if let Some(x_amz_content_sha256) = headers.get("x-amz-content-sha256"){
    x_amz_content_sha256.to_str().unwrap()
  } else {
    "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855" // Empty string SHA256 hashed
  };

  let mut date_key = HmacSha256::new_from_slice(format!("AWS4{}", client_secret).as_bytes())?;
  date_key.update(auth.date.as_bytes());
  let date_key = date_key.finalize_fixed();

  let mut date_region_key = HmacSha256::new_from_slice(&date_key)?;
  date_region_key.update(auth.region.as_bytes());
  let date_region_key = date_region_key.finalize_fixed();

  let mut date_region_service_key = HmacSha256::new_from_slice(&date_region_key)?;
  date_region_service_key.update(auth.service.as_bytes());
  let date_region_service_key = date_region_service_key.finalize_fixed();

  let mut signing_key = HmacSha256::new_from_slice(&date_region_service_key)?;
  signing_key.update("aws4_request".as_bytes());
  let signing_key = signing_key.finalize_fixed();

  let mut canonical_headers = vec![];
  let mut canonical_query = vec![];

  for (key, value) in headers{
    let key = key.to_string().to_lowercase();

    if auth.signed_headers.contains(&key){
      canonical_headers.push(format!("{}:{}\n", key, value.to_str().unwrap().trim()));
    }
  }

  for query in req.uri().query().unwrap_or("").split("&"){
    if query == ""{ continue; }
    if query.contains('='){
      canonical_query.push(query.to_owned());
    } else{
      canonical_query.push(format!("{query}="));
    }
  }

  canonical_headers.sort();
  canonical_query.sort();

  let canonical_request = format!(
    "{method}\n{uri}\n{query_string}\n{headers}\n{signed}\n{sha256}",
    method = req.method().as_str(),
    uri = req.uri().path(),
    query_string = canonical_query.join("&"),
    headers = canonical_headers.join(""),
    signed = auth.signed_headers.join(";"),
    sha256 = x_amz_content_sha256
  );

  let req_hash = hex::encode(Sha256::digest(canonical_request.as_bytes()));

  let string_to_sign = format!(
    "{CLIENT_ALGORITHM}\n{}\n{}/{}/s3/aws4_request\n{}",
    x_amz_date,
    auth.date, auth.region,
    req_hash
  );

  let mut signature = HmacSha256::new_from_slice(&signing_key)?;
  signature.update(string_to_sign.as_bytes());
  let signature = signature.finalize_fixed();

  let signature = hex::encode(signature);

  if auth.signature == signature{
    Ok(())
  } else{
    bail!("Invalid signature")
  }
}

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

fn parse_auth_header<'a>(auth: &'a str) -> anyhow::Result<AuthorizationHeader>{
  let mut auth = auth.split(' ');
  let Some(algorithm) = auth.nth(0) else { bail!("Cannot get Algorithm") };

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
        if cred.len() < 4 { bail!("Invalid Auth Header") }

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

  Ok(header)
}