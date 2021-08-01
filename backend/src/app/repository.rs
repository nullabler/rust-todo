use crate::models::{model::Model, posts::PostModel};

pub struct Repository {
    pub post: PostModel
}

impl Repository {
    pub fn new() -> Self {
        Repository {
            post: Model::new(),
        }
    }
}
