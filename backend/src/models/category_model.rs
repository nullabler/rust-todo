use diesel::prelude::*;

use super::Model;
use crate::app::Db;
use chrono::{NaiveDateTime, NaiveDate};
use crate::schema::category;

#[derive(Queryable, Insertable, Debug)]
#[table_name="category"]
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

    // pub fn create(&self, db: &Db) {
    //     // use crate::schema::category;

    //     let new_category = Category {
    //         id: 1,
    //         name: "qwe".to_string(),
    //         created_at: NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11),
    //     };

    //     diesel::insert_into(category::table)
    //         .values(&new_category)
    //         .get_result(&db.connect)
    //         .expect("Error saving new category");
    // }
}
