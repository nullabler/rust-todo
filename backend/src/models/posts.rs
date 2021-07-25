use diesel::prelude::*;

use diesel::mysql::MysqlConnection;
use diesel::Queryable;
use crate::app::db;

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
}

pub struct PostModel {
    connect: MysqlConnection,
}

impl PostModel {
    fn query(&self) { 
        use super::schema::posts::dsl::*;

        let connect: Result<MysqlConnection, ConnectionError> = db::Db::CONNECTION.connect;

        let results = posts
            .limit(5)
            .load::<Post>(&connect.unwrap())
            .expect("Error loading posts");

        println!("Displaying {} posts", results.len());
    }
}
//
// impl PostModel {
//     pub fn new(connect: MysqlConnection) -> Self {
//         PostModel {
//             connect
//         }
//     }

//     pub fn query(&self) {
//         use super::schema::posts::dsl::*;
//         let results = posts
//             .limit(5)
//             .load::<Post>(&self.connect)
//             .expect("Error loading posts");

//         println!("Displaying {} posts", results.len());
//     }
// }
