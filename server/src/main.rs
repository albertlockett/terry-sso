use actix_cors::Cors;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use base64;
use serde::{Deserialize};
use sha2::{Sha256, Digest};
use uuid::Uuid;

mod dao;
mod token;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let server = HttpServer::new(|| {
    let cors = Cors::permissive();
    App::new()
      .wrap(cors)
      .route("/oauth2/authorize", web::get().to(redir_to_login))
      .route("/oauth2/login", web::post().to(handle_login))
      .route("/oauth2/token", web::post().to(handle_token))
  });

  server.bind("127.0.0.1:4000")
    .expect("error binding server")
    .run()
    .await
}

#[derive(Debug, Deserialize)]
pub struct AuthorizeParams {
  challenge: String,
  callback_url: String,
}

async fn redir_to_login(req: HttpRequest) -> HttpResponse {
  let params = web::Query::<AuthorizeParams>::from_query(req.query_string()).unwrap();
  println!("challenge = {:?}", params.challenge);
  dao::store_challenge(&params.challenge).await;
  let redirect_location = format!(
      "http://localhost:1234?challenge={}&callbackUrl={}", 
      params.challenge, 
      params.callback_url
  );
  HttpResponse::Found()
    .header("Location", redirect_location)
    .finish()
}

#[derive(Debug, Deserialize)]
pub struct PasswordFormValues {
  username: String,
  password: String,
  challenge: String,
  callback_url: String,
}

async fn handle_login(params: web::Form::<PasswordFormValues>) -> HttpResponse {
  // check that the code is valid ...
  let challenge = dao::get_challenge(&params.challenge).await;
  if matches!(challenge, None) {
    // TODO test this
    // design decision - redirect to callback url to tell user they biffed it
    // or send them back to the login screen?
    let location = format!(
      "http://localhost:1234?challenge={}&callbackUrl={}&error={}", 
      params.challenge,
      params.callback_url,
      "invalid_challenge"
    );
    return HttpResponse::Found()
      .header("Location", location)
      .finish()
  }

  // check that the user supplied valid credentials
  // don't do this irl - it's not a very smart way to check creds
  let valid_credentials: bool;
  let password = dao::get_password(&params.username).await;
  println!("{:?}", password);

  if matches!(password, None) {
    valid_credentials = false; // no user found
  } else {
    valid_credentials = params.password == password.unwrap();
  }
  if valid_credentials == false {
    let location = format!(
      "http://localhost:1234?challenge={}&callbackUrl={}&error={}", 
      params.challenge,
      params.callback_url,
      "invalid_credentials"
    );
    return HttpResponse::Found()
      .header("Location", location)
      .finish()
  }
  
  let code = format!("{}", Uuid::new_v4());
  dao::store_code(&code, &params.challenge).await;
  HttpResponse::Found()
    .header("Location", format!("{}?code={}", params.callback_url.clone(), code))
    .finish()
}

#[derive(Debug, Deserialize)]
pub struct TokenFormValues {
  code: String,
  verifier: String
}

async fn handle_token(params: web::Json::<TokenFormValues>) -> HttpResponse {
  let code = dao::get_code(&params.code).await;
  if matches!(code, None) {
    return HttpResponse::BadRequest()
      .header("content-type", "application/json")
      .body("{\"error\": \"invalid_code\"}")
  }
  
  let challenge = code.unwrap().challenge;
  if !is_valid_verifier(&params.verifier, &challenge) {
    return HttpResponse::BadRequest()
      .header("content-type", "application/json")
      .body("{\"error\": \"invalid_verifier\"}")
  }

  let access_token = token::generate_token("cheese2");
  HttpResponse::Ok()
    .header("content-type", "application/json")
    .body(format!("{{\"access_token\": \"{}\"}}", access_token))
}


fn is_valid_verifier(verifier: &str, challenge: &str) -> bool {
  println!("{:?}", challenge);
  let decoded_challenge = base64::decode(challenge).unwrap();

  let mut hasher = Sha256::new();
  hasher.update(verifier);
  let result = hasher.finalize();

  println!("decoded challenge {:?}", decoded_challenge);
  println!("result = {:?}", result);

  result.to_vec() == decoded_challenge
}