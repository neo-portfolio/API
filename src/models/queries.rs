use crate::models::company::Company;
use crate::neo4j::structures::QueryResponse;
use crate::neo4j::transaction::{CypherParameter, CypherStatement, Transaction};

pub async fn companies_and_correlations(symbols: &Vec<String>) -> QueryResponse<Company> {
    let mut tx: Transaction = Transaction::new();
    let query_string = "WITH $SYMBOLs AS symbols\
    MATCH (c: Company)\
    WHERE c.symbol IN symbols\
    WITH collect(c) AS companies\
    UNWIND companies AS c1\
    MATCH (c1)-[r]-(c2)\
    WHERE c2 IN companies\
    RETURN c1, r, c2".to_string();
    let mut statement: CypherStatement = CypherStatement::new(query_string);
    let parameters: CypherParameter = {
        "SYMBOLS": symbols
    };
    statement.set_parameters(parameters);
    tx.add_statement(statement);
    return tx.commit().await;
}