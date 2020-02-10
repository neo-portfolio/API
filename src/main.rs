#[macro_use]
extern crate log;

use actix_web::{App, HttpServer};

use neo4j::driver::Driver;

mod config;
mod services;
mod neo4j;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let mut client: Driver = Driver::new("skyr.internet-box.ch", 7474, "neo4j");

    client.authentication("neo4j", "test1234");

    client.query("MATCH (n) RETURN n LIMIT 25").await;


    HttpServer::new(move || App::new()
        .configure(config::app::config))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}