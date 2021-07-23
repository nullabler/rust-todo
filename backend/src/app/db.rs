// use crate::models::posts::PostModel;
use diesel::prelude::*;
use crate::app::config::Config;

use diesel::mysql::MysqlConnection;


pub struct Db {
    connect: MysqlConnection,
    // post: PostModel
}

impl Db {
    pub fn new (config: &Config) -> Self {
        // let connect = MysqlConnection::establish(config.db_url()).expect(&format!("Error connecting to {}", config.db_url()));
        Db {
            connect: MysqlConnection::establish(config.db_url()).expect(&format!("Error connecting to {}", config.db_url())),
            // post: PostModel::new(&connect)
        }
    }
}
