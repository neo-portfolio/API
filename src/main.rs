#[macro_use]
extern crate log;

use actix_web::{App, HttpServer};

mod config;
mod services;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(move || App::new()
        .configure(config::app::config))
        .bind("127.0.0.1:8080")?
        .run()
        .await;
    return server;
}