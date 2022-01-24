use base64;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize};

#[derive(Debug, Serialize)]
struct  TokenHeader {
  alg: String,
  typ: String
}

#[derive(Debug, Serialize)]
struct TokenBody {
  iss: String,
  sub: String,
  iat: u64,
  exp: u64,
}

pub fn generate_token(subject: &str) -> String {
  let header = TokenHeader {
    alg: String::from("HS256"),
    typ: String::from("JWT")
  };

  let seconds_since_epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
  let body = TokenBody {
    iss: String::from("terrylockett.ca"),
    sub: String::from(subject),
    iat: seconds_since_epoch,
    exp: seconds_since_epoch + token_ttl(),
  };
  
  return format!(
    "{}.{}.todo-encrypted-footer", 
    base64::encode(json_serialize(header)),
    base64::encode(json_serialize(body))
  );
}

fn json_serialize<T: Serialize>(value: T) -> String {
  return serde_json::to_string(&value).unwrap();
}

fn token_ttl() -> u64 {
  30 * 60 * 1000
}