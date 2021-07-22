use diesel::prelude::*;

use diesel::Queryable;
use diesel::mysql::MysqlConnection;

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
}


pub fn query(connect: &MysqlConnection) {
    use super::schema::posts::dsl::*;
    let results = posts
        .limit(5)
        .load::<Post>(connect)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
}
