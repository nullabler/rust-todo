use std::env;

pub struct Config {
    addr: String,
    db_url: String,
}

impl Config {
    pub fn new () -> Self {
        Config {
            addr: env::var("ADDR").unwrap(),
            db_url: env::var("DATABASE_URL").unwrap(),
        }
    }

    pub fn addr(&self) -> &str {
        &self.addr.as_str()
    }

    pub fn db_url(&self) -> &str {
        &self.db_url.as_str()
    }
}
