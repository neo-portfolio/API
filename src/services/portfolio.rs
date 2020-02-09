use actix_web::{HttpResponse, web};

// POST api/address-book
async fn test() -> HttpResponse {
    return HttpResponse::Ok().body("app");
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test")
            .route(web::get().to(test))
    );
}