use std::env;

pub struct Config {
    addr: String,
    db_url: String,
}

impl Config {
    pub fn new() -> Self {
        Config {
            addr: env::var("ADDR").unwrap(),
            db_url: Config::get_db_url(),
        }
    }

    fn get_db_url() -> String {
        format!(
            "mysql://{user}:{password}@{host}:{port}/{database}",
            user = env::var("MYSQL_USER").unwrap(),
            password = env::var("MYSQL_PASSWORD").unwrap(),
            host = env::var("MYSQL_HOST").unwrap(),
            port = env::var("MYSQL_PORT").unwrap(),
            database = env::var("MYSQL_DATABASE").unwrap(),
        )
    }

    pub fn addr(&self) -> &str {
        &self.addr.as_str()
    }

    pub fn db_url(&self) -> &str {
        &self.db_url.as_str()
    }
}
