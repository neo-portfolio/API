use actix_web::{HttpResponse, web};

async fn test() -> HttpResponse {
    HttpResponse::Ok().body("app")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test")
            .route(web::get().to(test))
    );
}