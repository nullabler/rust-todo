// use crate::models::posts::PostModel;
use diesel::prelude::*;
use crate::app::config::Config;

use diesel::mysql::MysqlConnection;
// use std::sync::Arc;

pub struct Db {
    pub connect: MysqlConnection,
    // pub post: PostModel,
    // pub post_new: PostModel
}

impl Db {
    pub fn new (config: &Config) -> Self {
        // let connect = Arc::new(MysqlConnection::establish(config.db_url()).expect(&format!("Error connecting to {}", config.db_url())));
        Db {
            connect: MysqlConnection::establish(config.db_url()).expect(&format!("Error connecting to {}", config.db_url())),
            // post: PostModel::new(Rc::clone(&connect)),
            // post_new: PostModel::new(Rc::clone(&connect))
        }
    }

    // pub fn post(&self) -> PostModel {
    //     &self.post.init(&self.connect)
    // }
}
