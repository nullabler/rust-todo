use diesel::prelude::*;

use diesel::Queryable;
use super::Model;
use crate::app::Db;
use chrono::NaiveDateTime;

#[derive(Queryable, Debug)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
}

pub struct CategoryModel {}

impl Model for CategoryModel {
    fn new() -> Self {
        CategoryModel {}
    }
}

impl CategoryModel {
    pub fn query(&self, db: &Db) -> Vec<Category> {
        use crate::schema::category::dsl::*;

        let results: Vec<Category> = category
            .limit(5)
            .load(&db.connect)
            .expect("Error loading category");

        // results;
        // println!("{:?}", results.get(0));

        return results;
        // results;
        // println!("Displaying {} category", results.len());
    }
}
