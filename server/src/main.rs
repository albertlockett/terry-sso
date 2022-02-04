use actix_cors::Cors;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use base64;
use serde::Deserialize;
use sha2::{Digest, Sha256};
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

    server
        .bind("127.0.0.1:4000")
        .expect("error binding server")
        .run()
        .await
}

#[derive(Debug, Deserialize)]
pub struct AuthorizeParams {
    challenge: String,
    callback_url: String,
    audience: String,
    scopes: String,
}

async fn redir_to_login(req: HttpRequest) -> HttpResponse {
    let params = web::Query::<AuthorizeParams>::from_query(req.query_string()).unwrap();
    let session_id = Uuid::new_v4().to_string();
    dao::store_session(
        &session_id,
        dao::Session {
            challenge: params.challenge.clone(),
            callback_url: params.callback_url.clone(),
            audience: params.audience.clone(),
            scopes: params.scopes.clone(),
        },
    )
    .await;
    let redirect_location = format!("http://localhost:1234?session_id={}", session_id,);
    HttpResponse::Found()
        .header("Location", redirect_location)
        .finish()
}

#[derive(Debug, Deserialize)]
pub struct PasswordFormValues {
    username: String,
    password: String,
    session_id: String,
}

async fn handle_login(params: web::Form<PasswordFormValues>) -> HttpResponse {
    // check if the session is valid
    let session_op = dao::get_session(&params.session_id).await;
    if session_op.is_none() {
        let location = format!(
            "http://localhost:1234?session_id={}&error={}",
            params.session_id, "invalid_session"
        );
        return HttpResponse::Found().header("Location", location).finish();
    }
    let session = session_op.unwrap();

    // check if the password is valid ...
    if !is_valid_password(&params).await {
        let location = format!(
            "http://localhost:1234?session_id={}&error={}",
            params.session_id, "invalid_credentials"
        );
        return HttpResponse::Found().header("Location", location).finish();
    }

    println!("scopes = {:?}", session.scopes);
    println!("audience = {:?}", session.audience);

    // store the code and redirect user w/ code
    let code = format!("{}", Uuid::new_v4());
    dao::store_code(&code, &params.username, &params.session_id).await;
    let callback_url = format!("{}?code={}", session.callback_url.clone(), code);
    HttpResponse::Found()
        .header("Location", callback_url)
        .finish()
}

async fn is_valid_password(params: &web::Form<PasswordFormValues>) -> bool {
    let valid_credentials: bool;
    let password = dao::get_password(&params.username).await;
    println!("{:?}", password);

    if matches!(password, None) {
        valid_credentials = false; // no user found
    } else {
        valid_credentials = params.password == password.unwrap();
    }
    return valid_credentials;
}

#[derive(Debug, Deserialize)]
pub struct TokenFormValues {
    code: String,
    verifier: String,
}

async fn handle_token(params: web::Json<TokenFormValues>) -> HttpResponse {
    let code_op = dao::get_code(&params.code).await;
    if code_op.is_none() {
        return HttpResponse::BadRequest()
            .header("content-type", "application/json")
            .body("{\"error\": \"invalid_code\"}");
    }
    let code = code_op.unwrap();

    let session_id = code.session_id;
    let session_op = dao::get_session(&session_id).await;
    if session_op.is_none() {
        // biffed ITTTTTTT
        return HttpResponse::BadRequest()
            .header("content-type", "application/json")
            .body("{\"error\": \"invalid_state\"}");
    }
    let session = session_op.unwrap();

    // TODO need to fix all this stuff ...
    let challenge = session.challenge;
    if !is_valid_verifier(&params.verifier, &challenge) {
        return HttpResponse::BadRequest()
            .header("content-type", "application/json")
            .body("{\"error\": \"invalid_verifier\"}");
    }

    let access_token = token::generate_token(&code.user);
    HttpResponse::Ok()
        .header("content-type", "application/json")
        .body(format!("{{\"access_token\": \"{}\"}}", access_token))
}

fn is_valid_verifier(verifier: &str, challenge: &str) -> bool {
    let decoded_verifier = base64::decode(verifier).unwrap();
    let mut hasher = Sha256::new();
    hasher.update(decoded_verifier);
    let verifier_hashed_b64 = base64::encode(hasher.finalize());
    verifier_hashed_b64 == challenge
}
