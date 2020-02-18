use crate::models::company::{CompanyOrCorrelation};
use crate::neo4j::structures::QueryResponse;
use crate::neo4j::transaction::{CypherParameter, CypherStatement, Transaction};
use serde_json::json;

pub async fn companies_and_correlations(symbols: &Vec<String>) -> QueryResponse<CompanyOrCorrelation> {
    let mut tx: Transaction = Transaction::new();
    let query_string = "WITH $SYMBOLS AS symbols
    MATCH (c: Company)
    WHERE c.symbol IN symbols
    WITH collect(c) AS companies
    UNWIND companies AS c1
    MATCH (c1)-[r]-(c2)
    WHERE c2 IN companies
    RETURN c1, r.corr, c2".to_string();
    let mut statement: CypherStatement = CypherStatement::new(query_string);
    let mut parameters: CypherParameter = CypherParameter::new();
    parameters.insert("SYMBOLS".to_string(), json!(symbols));
    statement.set_parameters(parameters);
    tx.add_statement(statement);
    return tx.commit().await;
}