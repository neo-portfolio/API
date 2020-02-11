use actix_web::{HttpResponse, web};
use serde::Deserialize;
use crate::models::company::Company;
use crate::neo4j::driver::Driver;

#[derive(Deserialize)]
pub struct CompanyRequest {
    symbol: String
}

async fn company_info(web::Query(params): web::Query<CompanyRequest>) -> HttpResponse {
    let mut client: Driver = Driver::new("skyr.internet-box.ch", 7474, "neo4j");
    client.authentication("neo4j", "test1234");
    let query = format!("MATCH (company: Company {{symbol: '{}'}}) RETURN company;", params.symbol);
    let object = client.query::<Company>(&query).await;
    let data = object.data();
    HttpResponse::Ok().json(data)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/company_info")
            .route(web::get().to(company_info))
    );
}