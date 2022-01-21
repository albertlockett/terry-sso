use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer};

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

fn redir_to_login() -> HttpResponse {
  HttpResponse::Found()
    .header("Location", "http://localhost:1234")
    .finish()
}