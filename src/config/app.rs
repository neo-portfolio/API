use actix_web::web;

use crate::services;

pub fn config(cfg: &mut web::ServiceConfig) {
    info!("Configurating routes...");
    cfg.service(
        web::scope("/api")
            .service(web::scope("/portfolio").configure(services::portfolio::config))
    );
}