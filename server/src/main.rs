use actix_cors::Cors;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use serde::{Deserialize};
use uuid::Uuid;

mod dao;

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
  HttpResponse::Found()
    .header("Location", format!("http://localhost:1234?challenge={}&callbackUrl={}", params.challenge, params.callback_url))
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
  // TODO check if the challenge exists
  // TODO check if the username and password is correct
  let code = format!("{}", Uuid::new_v4());
  dao::store_code(&code, &params.challenge).await;
  HttpResponse::Found()
    .header("Location", format!("{}?code={}", params.callback_url.clone(), code))
    .finish()
}


pub fn handle_token() -> HttpResponse {
  HttpResponse::Ok().body("all good bb")
}