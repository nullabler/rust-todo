use crate::models::{CategoryModel, TaskModel, Model};

pub struct Repository {
    pub category: CategoryModel,
    pub task: TaskModel,
}

impl Repository {
    pub fn new() -> Self {
        Repository {
            category: Model::new(),
            task: Model::new(),
        }
    }
}
