use actix_web::client::Client;

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

    pub async fn query(&self, statement: &str) {
        let client = Client::default();

        let json = self.query_builder(statement);
        let url = format!("http://{}:{}/db/{}/tx/commit", self.host, self.port, self.db);
        println!("{}", url);

        let response = client.post(url)
                             .header("Content-Type", "application/json")
                             .header("Authorization", format!("{} {}", self.user, self.password))
                             .send_json(&json)
                             .await;                      // <- Send http request

        println!("Response: {:?}", response);
    }
}