use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct JumpLabel {
    pub name: String,
}

impl JumpLabel {
    pub fn new(name: String) -> JumpLabel {
        JumpLabel { name: name }
    }
}

#[derive(Debug, Clone)]
pub struct JumpLabelsMap {
    pub map: HashMap<String, JumpLabel>,
}

impl JumpLabelsMap {
    pub fn new() -> JumpLabelsMap {
        JumpLabelsMap {
            map: HashMap::new(),
        }
    }

    pub fn add(&mut self, name: String) {
        self.map.insert(name.clone(), JumpLabel::new(name));
    }

    pub fn get(&self, name: &str) -> Option<&JumpLabel> {
        self.map.get(name)
    }
}
