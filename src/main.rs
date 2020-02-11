#[macro_use]
extern crate log;

use actix_cors::Cors;
use actix_web::{App, HttpServer};

mod config;
mod services;
mod neo4j;
pub mod models;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("START");
    HttpServer::new(move || App::new()
        .wrap(Cors::new()
            .allowed_origin("http://localhost:3000")
            .max_age(3600)
            .finish()
        )
        .configure(config::app::config))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}