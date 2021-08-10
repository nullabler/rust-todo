use diesel::prelude::*;

use diesel::Queryable;
use chrono::NaiveDateTime;
use super::Model;
use crate::app::Db;

#[derive(Queryable)]
pub struct Task {
    pub id: i32,
    pub category_id: i32,
    pub name: String,
    pub status: String,
    pub created_at: NaiveDateTime,
}

pub struct TaskModel;

impl Model for TaskModel {
    fn new() -> Self {
        TaskModel {}
    }
}

impl TaskModel {
    pub fn query(&self, db: &Db) {
        use crate::schema::task::dsl::*;

        let results = task
            .limit(5)
            .load::<Task>(&db.connect)
            .expect("Error loading task");

        println!("Displaying {} task", results.len());
    }
}
