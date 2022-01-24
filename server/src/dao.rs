use s3::bucket::Bucket;
use s3::creds::Credentials;
use serde::{Deserialize, Serialize};
use serde_json;

/**
 * I realize this is a funny implementation of a database for our sensitive data
 * but when I put the question to a vote about what we could use to store this
 * stuff, everyone voted for public S3 bucket. 
 */

#[derive(Debug, Deserialize, Serialize)]
pub struct Challenge {}

#[derive(Debug, Deserialize, Serialize)]
pub struct Code {
  pub challenge: String,
}

pub async fn store_challenge(challenge: &str) {
  let bucket = get_bucket();
  let content = serde_json::to_string(&Challenge{}).unwrap();
  let result = bucket.put_object(format!("{}", challenge), content.as_bytes()).await;
  println!("{:?}", result)
}

pub async fn get_challenge(challenge: &str) -> Option<Challenge> {
  let bucket = get_bucket();
  let result = bucket.get_object(format!("{}", challenge)).await;
  match result {
    Ok((data, 200)) => { 
      let challenge = serde_json::from_slice(&data).unwrap();
      Some(challenge)
    },
    _ => { None }
  }
}

pub async fn store_code(code: &str, challenge: &str) {
  let bucket = get_bucket();
  let content = serde_json::to_string(&Code {
    challenge: String::from(challenge),
  }).unwrap();
  let result = bucket.put_object(format!("{}", code), content.as_bytes()).await;
  println!("{:?}", result)
}

pub async fn get_code(code: &str) -> Option<Code> {
  let bucket = get_bucket();
  let result = bucket.get_object(format!("{}", code)).await;
  match result {
    Ok((data, 200)) => {
      let code = serde_json::from_slice(&data).unwrap();
      Some(code)
    },
    _ => { None }
  }
}

pub async fn get_password(username: &str) -> Option<String> {
  let bucket = get_bucket();
  let result = bucket.get_object(format!("{}", username)).await;
  match result {
    // return option w/ the password if we were able to fetch it
    Ok((data, 200)) => {
      let password = std::str::from_utf8(&data).unwrap();
      Some(String::from(password))
    }
    // for any non 200 response code or any errors, return not the password
    Ok((_,_)) => { None }
    Err(_) => { None }
  }
}

fn get_bucket() -> Bucket {
  let region = "us-east-1".parse().unwrap();
  let credentials = Credentials::default().unwrap();
  let bucket = Bucket::new("terrylockett-sso", region, credentials).unwrap();
  return bucket;
}