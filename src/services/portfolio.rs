use actix_web::{HttpResponse, web};
use serde::Deserialize;

use crate::models::company::Company;
use crate::neo4j::transaction::Transaction;

#[derive(Deserialize)]
struct CompanyRequest {
    symbol: String
}

#[derive(Deserialize)]
struct GenerateRequestBody {
    companies: Vec<String>,
    #[serde(rename = "maxCount")]
    max_count: i16,
}


async fn company_info(web::Query(params): web::Query<CompanyRequest>) -> HttpResponse {
    //let sub_query = params.symbol.split(",").join(",");
    let query_str = format!("MATCH (company: Company) WHERE company.symbol IN [{}] RETURN company;", params.symbol);
    let mut tx = Transaction::new();
    tx.add_statement(query_str);
    let object = tx.commit::<Company>().await;
    let data = object.data();
    HttpResponse::Ok().json(data)
}


async fn generate_portfolio(body: web::Json<GenerateRequestBody>) -> HttpResponse {
    HttpResponse::Ok().body("")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/company_info").route(web::get().to(company_info)))
    .service(web::resource("/generate").route(web::post().to(generate_portfolio)))
    ;
}