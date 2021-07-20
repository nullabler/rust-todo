use std::env;

pub struct Config {
    addr: String,
    db_url: String,
}

impl Config {
    pub fn new () -> Self {
        Config {
            addr: env::var("ADDR").unwrap(),
            db_url: Config::get_db_url(),
        }
    }

    fn get_db_url() -> String {
        let mut db_url = String::new();
        db_url.push_str("mysql://");
        db_url.push_str(env::var("MYSQL_USER").unwrap().as_str());
        db_url.push(':');
        db_url.push_str(env::var("MYSQL_PASSWORD").unwrap().as_str());
        db_url.push_str("@");
        db_url.push_str(env::var("MYSQL_HOST").unwrap().as_str());
        db_url.push(':');
        db_url.push_str(env::var("MYSQL_PORT").unwrap().as_str());
        db_url.push('/');
        db_url.push_str(env::var("MYSQL_DATABASE").unwrap().as_str());
        db_url
    }

    pub fn addr(&self) -> &str {
        &self.addr.as_str()
    }

    pub fn db_url(&self) -> &str {
        &self.db_url.as_str()
    }
}
