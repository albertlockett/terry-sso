use actix_cors::Cors;
use actix_web::{web, App, FromRequest, HttpRequest, HttpResponse, HttpServer};
use serde::{Deserialize};

mod dao;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let server = HttpServer::new(|| {
    let cors = Cors::permissive();
    App::new()
      .wrap(cors)
      .route("/oauth2/authorize", web::get().to(redir_to_login))
      .route("/oauth2/login", web::post().to(handle_login))
  });

  server.bind("127.0.0.1:4000")
    .expect("error binding server")
    .run()
    .await
}

#[derive(Debug, Deserialize)]
pub struct AuthorizeParams {
  challenge: String,
}

async fn redir_to_login(req: HttpRequest) -> HttpResponse {
  let params = web::Query::<AuthorizeParams>::from_query(req.query_string()).unwrap();
  println!("challenge = {:?}", params.challenge);
  dao::store_challenge(&params.challenge).await;
  HttpResponse::Found()
    .header("Location", format!("http://localhost:1234?challenge={}", params.challenge))
    .finish()
}

#[derive(Debug, Deserialize)]
pub struct PasswordFormValues {
  username: String,
  password: String,
  challenge: String,
}

async fn handle_login(params: web::Form::<PasswordFormValues>) -> HttpResponse {
  println!("{:?}", params);
  HttpResponse::Found()
    .header("Location", "http://localhost:1234")
    .finish()
}
