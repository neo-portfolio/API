pub struct Driver<'a> {
    // Static immutable -> no need for getters and setters
    pub host: &'a str,
    pub port: i16,
    pub db: &'a str,
    pub user: &'a str,
    pub password: &'a str,
}

pub static DRIVER: Driver = Driver { host: "skyr.internet-box.ch", port: 7474, db: "neo4j", user: "neo4j", password: "test1234" };