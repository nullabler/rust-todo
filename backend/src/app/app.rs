use crate::models::model::Model;
use super::{cache::Cache, config::Config, db::Db, repository::Repository};

pub struct App {
    pub config: Config,
    pub cache: Cache,
    pub db: Db,
    pub repository: Repository
}

impl App {
    pub fn new() -> App {
        let config = Config::new();
        let db = Db::new(&config);

        let repository = Repository {
            post: Model::new()
        };

        App {
            config,
            cache: Cache::new(),
            db,
            repository,
        }
    }
}
