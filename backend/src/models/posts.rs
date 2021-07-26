use diesel::prelude::*;

use diesel::Queryable;
use diesel::mysql::MysqlConnection;
// use crate::app::db::Db;
// use std::cell::RefCell;
use std::sync::{Mutex, Arc};
use crate::app::Db;
use super::model::Model;


#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
}

pub struct PostModel {
    db: Arc<Mutex<Db>>
}

impl Model for PostModel {
    fn new(db: Arc<Mutex<Db>>) -> Self {
        PostModel {
            db
        }
    }

    // fn db(&self) -> &Arc<Mutex<Db>> {
    //     &self.db
    // }
}

impl PostModel {
    // pub fn query(&self) {
    //     let _q = &*db.connect;
        // println!("q2e: {:?}", _q);
    // }

    pub fn query(&self) {
        use super::schema::posts::dsl::*;

        // let reference = &self.reference();
        // let db: &Db = reference.as_ref();

        // let connect = &self.db.lock().unwrap().connect;
        let results = posts
            .limit(5)
            .load::<Post>(&self.db.lock().unwrap().connect)//&*db.connect)//&self.reference().connect)
            .expect("Error loading posts");

        println!("Displaying {} posts", results.len());
    }


    // pub fn query_2(&self, db: &Db) {
    //     use super::schema::posts::dsl::*;

        // let reference = &self.reference();
        // let db: &Db = reference.as_ref();

        // let connect = &self.db.lock().unwrap().connect;
    //     let results = posts
    //         .limit(5)
    //         .load::<Post>(db.connect)//&*db.connect)//&self.reference().connect)
    //         .expect("Error loading posts");

    //     println!("Displaying {} posts", results.len());
    // }
}
