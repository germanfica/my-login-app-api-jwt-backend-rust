// use actix_web::{App, HttpServer};
//use actix_identity::{IdentityService, CookieIdentityPolicy};
use crate::config::Config;
use actix_cors::Cors;
use actix_identity::IdentityMiddleware;
use actix_web::middleware::Logger;
use env_logger::Env;

// use crate::auth::{login, authenticate_jwt};

use actix_web::{cookie::Key, get, post, web, App, HttpResponse, HttpServer, Responder};

// mod auth;
mod config;
// mod services;

#[get("/")]
async fn hello() -> impl Responder {
    let config = Config::from_env();

    let response_body = format!(
        "Hello world! JWT: {}; DB_HOST: {}; DB_PORT: {}, DB_USERNAME: {}; DB_PASSWORD: {}; DB_NAME: {}",
        config.jwt_secret,
        config.db_host,
        config.db_port,
        config.db_username,
        config.db_password,
        config.db_name
    );

    HttpResponse::Ok().body(response_body)
}

#[get("/hello_world")]
async fn helloWorld() -> impl Responder {
    let config = Config::from_env();

    let response_body = format!(
        "Hello world! JWT: {}; DB_HOST: {}; DB_PORT: {}, DB_USERNAME: {}; DB_PASSWORD: {}; DB_NAME: {}",
        config.jwt_secret,
        config.db_host,
        config.db_port,
        config.db_username,
        config.db_password,
        config.db_name
    );

    HttpResponse::Ok().body(response_body)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_env();
    // let secret_key: Key = Key::from(config.jwt_secret.as_bytes()); // Usa la clave JWT como clave secreta para SessionMiddleware
    use actix_web::{App, HttpServer};
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:4200")
            .allow_any_header()
            .allow_any_method()
            .supports_credentials();

        App::new()
            .app_data(web::Data::new(config.clone())) // Use app_data here
            .wrap(cors)
            .wrap(IdentityMiddleware::default()) // Use IdentityMiddleware
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(hello)
            .service(echo)
            .service(helloWorld)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
