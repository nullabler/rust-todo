use std::sync::{Arc, Mutex};

use crate::models::{model::Model, posts::PostModel};

use super::{cache::Cache, config::Config, db::Db, repository::Repository};

pub struct App {
    pub config: Config,
    pub cache: Cache,
    // pub db: Db,
    pub repository: Repository
}

impl App {
    pub fn new() -> App {
        let config = Config::new();
        let db = Db::new(&config);
        let arc_db = Arc::new(Mutex::new(db));

        // let weak = Arc::downgrade(&arc_db);
        // let _post: PostModel = Model::new(weak);

        // _post.query();
        let repository = Repository {
            post: Model::new(Arc::clone(&arc_db))//Arc::downgrade(&arc_db))
        };

        App {
            config,
            cache: Cache::new(),
            // db,
            repository,
        }
    }
}
