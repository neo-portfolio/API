use actix_web::{HttpResponse, web};
use serde::Deserialize;
use serde_json::Value;

use crate::models::company::Company;
use crate::neo4j::transaction::{CypherStatement, Transaction};

#[derive(Deserialize)]
struct CompanyRequest {
    symbol: String
}

async fn company_info(web::Query(params): web::Query<CompanyRequest>) -> HttpResponse {
    let mut tx: Transaction = Transaction::new();
    let query_str: String = "MATCH (company: Company) WHERE company.symbol IN $symbols RETURN company;".to_string();
    let mut statement = CypherStatement::new(query_str);
    let mut parameter: serde_json::Map<String, Value> = serde_json::Map::<String, Value>::new();
    let sub_query: Vec<&str> = params.symbol.split(",").collect();
    parameter.insert("symbols".to_string(), Value::from(sub_query));
    statement.set_parameters(parameter);
    tx.add_statement(statement);
    let object = tx.commit::<Company>().await;
    let data = object.data();
    HttpResponse::Ok().json(data)
}

#[derive(Deserialize)]
struct GenerateRequestBody {
    symbols: Vec<String>,
    #[serde(rename = "maxCount")]
    max_count: i16,
}

async fn generate_portfolio(body: web::Json<GenerateRequestBody>) -> HttpResponse {
    let symbols = &body.symbols;
    let count = &body.max_count;

    HttpResponse::Ok().body("")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/company_info").route(web::get().to(company_info)))
       .service(web::resource("/generate").route(web::post().to(generate_portfolio)))
    ;
}