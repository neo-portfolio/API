use std::time::Duration;

use actix_web::client::ClientBuilder;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;

use crate::neo4j::driver::DRIVER;
use crate::neo4j::structures::QueryResponse;

pub type CypherParameter = serde_json::Map<String, Value>;

#[macro_export]
macro_rules! cypher_parameter {
    ({$($key: expr => $value: expr), +}) => {{
        use serde_json::json;
        let mut param = CypherParameter::new();
        $(
            param.insert($key, json!($value));
        )+
        param
    }};
}

#[derive(Debug, Serialize)]
pub struct CypherStatement {
    statement: String,
    parameters: CypherParameter,
}

#[derive(Debug, Serialize)]
pub struct Transaction {
    statements: Vec<CypherStatement>
}

impl CypherStatement {
    pub fn new(statement: String) -> CypherStatement {
        CypherStatement { statement, parameters: serde_json::Map::new() }
    }

    pub fn set_parameters(&mut self, parameters: CypherParameter) {
        self.parameters = parameters;
    }
}

impl Transaction {
    pub fn new() -> Transaction {
        Transaction { statements: Vec::new() }
    }

    pub fn add_statement(&mut self, statement: CypherStatement) {
        self.statements.push(statement);
    }

    pub async fn commit<T: DeserializeOwned>(&self) -> QueryResponse<T> {
        let timeout: Duration = Duration::new(20, 0);
        let client_builder = ClientBuilder::new().timeout(timeout);
        let client = client_builder.finish();

        let url = format!("http://{}:{}/db/{}/tx/commit", DRIVER.host, DRIVER.port, DRIVER.db);

        let mut response = client.post(url)
                                 .header("Content-Type", "application/json")
                                 .header("Authorization", format!("{} {}", DRIVER.user, DRIVER.password))
                                 .send_json(&self)
                                 .await
                                 .unwrap();


        let body = response.body().await.unwrap();
        let body_str = std::str::from_utf8(&body).unwrap();
        serde_json::from_str(body_str).unwrap()
    }
}