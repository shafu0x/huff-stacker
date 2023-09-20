use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Jump {
    pub name: String,
}

impl Jump {
    pub fn new(name: String) -> Jump {
        Jump { name: name }
    }
}

#[derive(Debug, Clone)]
pub struct JumpsMap {
    pub map: HashMap<String, Jump>,
}

impl JumpsMap {
    pub fn new() -> JumpsMap {
        JumpsMap {
            map: HashMap::new(),
        }
    }

    pub fn add(&mut self, name: String) {
        self.map.insert(name.clone(), Jump::new(name));
    }

    pub fn get(&self, name: &str) -> Option<&Jump> {
        self.map.get(name)
    }
}
