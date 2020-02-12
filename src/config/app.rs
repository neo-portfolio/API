use actix_web::web;

use crate::services::portfolio;

pub fn config(cfg: &mut web::ServiceConfig) {
    info!("Configurating routes...");
    cfg.service(
        web::scope("/api")
            .service(web::scope("/portfolio").configure(portfolio::config))
    );
}