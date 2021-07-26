use diesel::prelude::*;

use diesel::Queryable;
use crate::app::Db;
use super::model::Model;


#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
}

pub struct PostModel {}

impl Model for PostModel {
    fn new() -> Self {
        PostModel {}
    }
}

impl PostModel {

    pub fn query(&self, db: &Db) {
        use super::schema::posts::dsl::*;

        let results = posts
            .limit(5)
            .load::<Post>(&db.connect)
            .expect("Error loading posts");

        println!("Displaying {} posts", results.len());
    }
}
