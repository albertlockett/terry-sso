use actix_cors::Cors;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use serde::{Deserialize};

mod dao;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let server = HttpServer::new(|| {
    let cors = Cors::permissive();
    App::new()
      .wrap(cors)
      .route("/oauth2/authorize", web::get().to(redir_to_login))
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

fn redir_to_login(req: HttpRequest) -> HttpResponse {
  let params = web::Query::<AuthorizeParams>::from_query(req.query_string()).unwrap();
  println!("challenge = {:?}", params.challenge);
  dao::store_challenge(&params.challenge);
  HttpResponse::Found()
    .header("Location", "http://localhost:1234")
    .finish()
}

