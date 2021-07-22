// extern crate diesel;
// extern crate dotenv;

use crate::models::posts::query as queryR;
use diesel::prelude::*;
use crate::app::config::Config;

use diesel::mysql::MysqlConnection;


pub struct Db {
    connect: MysqlConnection
}

impl Db {
    pub fn new (config: &Config) -> Self {
        Db {
            connect: MysqlConnection::establish(config.db_url()).expect(&format!("Error connecting to {}", config.db_url()))
        }
    }

    pub fn query(&self) {
        queryR(&self.connect);
        // use super::schema::posts::dsl::*;
        // let results = posts
        //     .limit(5)
        //     .load::<Post>(&self.connect)
        //     .expect("Error loading posts");

        // println!("Displaying {} posts", results.len());
    }
}
