use actix_web::{HttpResponse, web};
use serde::Deserialize;
use crate::models::company::Company;
use crate::neo4j::transaction::{CypherStatement, Transaction, CypherParameter};
use crate::algorithms::low_beta::optimize_portfolio;
use crate::cypher_parameter;

#[derive(Deserialize)]
struct CompanyRequest {
    symbol: String
}

async fn company_info(web::Query(params): web::Query<CompanyRequest>) -> HttpResponse {
    let mut tx: Transaction = Transaction::new();
    let query_str: String = "MATCH (company: Company) WHERE company.symbol IN $symbols RETURN company;".to_string();
    let mut statement = CypherStatement::new(query_str);
    let sub_query: Vec<&str> = params.symbol.split(",").collect();
    let parameter: CypherParameter = cypher_parameter!({"symbols".to_string() => sub_query});
    statement.set_parameters(parameter);
    tx.add_statement(statement);
    let object = tx.commit::<Company>().await;
    let data = object.data();
    HttpResponse::Ok().json(data)
}

#[derive(Deserialize)]
struct GenerateRequestBody {
    symbols: Vec<String>
}

async fn generate_portfolio(body: web::Json<GenerateRequestBody>) -> HttpResponse {
    let symbols = &body.symbols;
    //let count = &body.max_count;
    let optimized_portfolio = optimize_portfolio(symbols).await;
    HttpResponse::Ok().json(optimized_portfolio)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/company_info").route(web::get().to(company_info)))
       .service(web::resource("/optimize").route(web::post().to(generate_portfolio)))
    ;
}