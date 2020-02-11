use actix_web::client::Client;

use crate::neo4j::structures::QueryResponse;
use serde::de::DeserializeOwned;

pub struct Driver<'a> {
    host: &'a str,
    port: i16,
    db: &'a str,
    user: &'a str,
    password: &'a str,
}

impl<'a> Driver<'a> {
    pub fn new(host: &'a str, port: i16, db: &'a str) -> Driver<'a> {
        Driver { host, port, db, user: "", password: "" }
    }

    pub fn authentication(&mut self, user: &'a str, password: &'a str) {
        self.user = user;
        self.password = password;
    }

    fn query_builder(&self, statement: &str) -> String {
        format!("{{\"statements\":[{{\"statement\": \"{}\"}}]}}", statement)
    }

    pub async fn query<T: DeserializeOwned>(&self, statement: &str) -> QueryResponse<T> {
        let client = Client::default();

        let json = self.query_builder(statement);
        let url = format!("http://{}:{}/db/{}/tx/commit", self.host, self.port, self.db);

        println!("{}", json);

        let mut response = client.post(url)
                             .header("Content-Type", "application/json")
                             .header("Authorization", format!("{} {}", self.user, self.password))
                             .send_body(&json)
                             .await.unwrap();


        let body = response.body().await.unwrap();
        let body_str = std::str::from_utf8(&body).unwrap();
        println!("{}", body_str);
        serde_json::from_str(body_str).unwrap()
    }
}