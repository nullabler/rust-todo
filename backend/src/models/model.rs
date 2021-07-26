// use std::cell::RefCell;
// use std::sync::{Arc, Mutex};
// use crate::app::Db;

pub trait Model {
    fn new() -> Self;

    // fn db(&self) -> &Arc<Mutex<Db>>;

    // fn reference(&self) -> Db {
    //     let db = self.db();
    //     *db.lock().unwrap()
        // self.db().borrow().upgrade().unwrap()
    // }
}
