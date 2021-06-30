pub struct Config {
    addr: String
}

impl Config {
    pub fn new () -> Self {
        Config {
            addr: "0.0.0.0:7878".to_string()
        }
    }

    pub fn addr(&self) -> &str {
        &self.addr.as_str()
    }
}
