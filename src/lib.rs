//! lib.rs

use actix_web::dev::Server;
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[post("/subscriptions")]
async fn subscriptions() -> impl Responder {
    HttpResponse::Ok()
}

// rustfmt::skip make sur `cargo fmt`skip this function
#[rustfmt::skip]
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .service(health_check)
            .service(subscriptions)
        })
        .listen(listener)?
        .run();

    Ok(server)
}
