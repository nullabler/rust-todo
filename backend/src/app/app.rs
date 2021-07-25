use super::{cache::Cache, config::Config, db::Db};

pub struct App {
    pub config: Config,
    pub cache: Cache,
    // pub db: Db,
}

impl super::db::Db for App {
    fn new() -> App {
        let config = Config::new();
        // let db = Db::new(&config);
        let mut db = Self::CONNECTION;
        db.set_connect(&config);
        App {
            config,
            cache: Cache::new(),
            // db,
        }
    }
}
