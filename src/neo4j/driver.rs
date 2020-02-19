use load_dotenv::load_dotenv;

pub struct Driver<'a> {
    // Static immutable -> no need for getters and setters
    pub host: &'a str,
    pub port: &'a str,
    pub db: &'a str,
    pub user: &'a str,
    pub password: &'a str,
}

load_dotenv!();

pub static DRIVER: Driver = Driver { host: env!("NEO4J_URL"), port: env!("NEO4J_PORT"), db: env!("NEO4J_PORT"), user: env!("NEO4J_USER"), password: env!("NEO4J_PASSWORD") };