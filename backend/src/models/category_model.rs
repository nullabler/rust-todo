use diesel::prelude::*;

use serde::{Serialize, Deserialize};
use diesel::Queryable;
use super::Model;
use crate::app::Db;

table! {
    category (id) {
        id -> Integer,
        name -> Varchar,
        created_at -> Varchar,
    }
}


#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub created_at: String,
}

pub struct CategoryModel {}

impl Model for CategoryModel {
    fn new() -> Self {
        CategoryModel {}
    }
}

impl CategoryModel {
    pub fn query(&self, db: &Db) -> Vec<Category> {
        use super::category_model::category::dsl::*;

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
