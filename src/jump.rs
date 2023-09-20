use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct JumpLabel {
    pub name: String,
    pub start: usize,
}

impl JumpLabel {
    pub fn new(name: String, start: usize) -> JumpLabel {
        JumpLabel {
            name: name,
            start: 0,
        }
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

    pub fn add(&mut self, jump_label: JumpLabel) {
        self.map.insert(jump_label.name.clone(), jump_label);
    }

    pub fn get(&self, name: &str) -> Option<&JumpLabel> {
        self.map.get(name)
    }
}
