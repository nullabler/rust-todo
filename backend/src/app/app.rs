use super::{cache::Cache, config::Config};

pub struct App {
    pub config: Config,
    pub cache: Cache,
}

impl App {
    pub fn new() -> App {
        App {
            config: Config::new(),
            cache: Cache::new(),
        }
    }
}
