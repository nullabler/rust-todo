use diesel::prelude::*;

use diesel::Queryable;
use super::Model;
use crate::app::Db;

table! {
    task (id) {
        id -> Int4,
        category_id -> Int4,
        name -> Varchar,
        status -> Varchar,
        created_at -> Varchar,
    }
}

#[derive(Queryable)]
pub struct Task {
    pub id: i32,
    pub category_id: i32,
    pub name: String,
    pub status: String,
    pub created_at: String,
}

pub struct TaskModel;

impl Model for TaskModel {
    fn new() -> Self {
        TaskModel {}
    }
}

impl TaskModel {
    pub fn query(&self, db: &Db) {
        use super::task_model::task::dsl::*;

        let results = task
            .limit(5)
            .load::<Task>(&db.connect)
            .expect("Error loading task");

        println!("Displaying {} task", results.len());
    }
}
