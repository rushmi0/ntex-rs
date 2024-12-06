use ntex::web;
use ntex_cors::Cors;
use ntex::http;

use crate::services;

pub async fn run() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "ntex=info");
    env_logger::init();
    web::HttpServer::new(|| web::App::new()
        .wrap(configure_cors().finish())
        .configure(services::init_services))
        .bind(("localhost", 8080))?
        .run()
        .await
}

fn configure_cors() -> Cors {
    Cors::new()
        .allowed_origin("*")
        .allowed_methods(vec!["GET", "POST"])
        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        .allowed_header(http::header::CONTENT_TYPE)
        .max_age(3600)
        .send_wildcard()
}