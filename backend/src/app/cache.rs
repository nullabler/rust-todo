use std::collections::HashMap;

#[derive(Debug)]
pub struct Cache {
    response: HashMap<String, String>,
    pub v: Vec<i32>,
}

impl Cache {
    pub fn new () -> Self {
        Cache {
            response: HashMap::new(),
            v: Vec::new(),
        }
    }
}
