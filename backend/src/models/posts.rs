use diesel::prelude::*;

use diesel::Queryable;
use diesel::mysql::MysqlConnection;
// use crate::app::db::Db;

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
}

pub struct PostModel {
    connect: MysqlConnection
}

impl PostModel {
    pub fn new(connect: MysqlConnection) -> Self {
        PostModel {
            connect
        }
    }

    pub fn query(&self) {
        use super::schema::posts::dsl::*;
        let results = posts
            .limit(5)
            .load::<Post>(&self.connect)
            .expect("Error loading posts");

        println!("Displaying {} posts", results.len());
    }
}
