use super::{cache::Cache, config::Config, db::Db};

pub struct App {
    pub config: Config,
    pub cache: Cache,
    pub db: Db,
}

impl App {
    pub fn new() -> App {
        let config = Config::new();
        let db = Db::new(&config);
        App {
            config,
            cache: Cache::new(),
            db,
        }
    }
}
