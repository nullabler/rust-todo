use std::collections::HashMap;

pub struct Cache {
    list: HashMap<&str, i32>
}

struct Chunk {

}

impl Cache {
    pub fn new() -> Self {
        Cache {
            list: &mut HashMap::new()
        }
    }

    pub fn get(&self, key: &str) -> Option<i32, None> {
        &self.list.get(key)
    }

    pub fn add(&self, key: &str, val: i32) {
        &self.list.entry(key).or_insert(val);
    }
}
