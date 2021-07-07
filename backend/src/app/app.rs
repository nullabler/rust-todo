use super::{cache::Cache, config::Config, db::Db};

pub struct App {
    pub config: Config,
    pub cache: Cache,
    pub db: Db,
}

impl App {
    pub fn new() -> App {
        App {
            config: Config::new(),
            cache: Cache::new(),
            db: Db::new("mysql://root:root@mysql:3306/todo"),
        }
    }
}
