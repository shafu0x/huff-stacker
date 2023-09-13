use std::fmt;

#[derive(Debug)]
pub struct Stack {
    values: Vec<String>,
}

impl fmt::Display for Stack {
fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let values_str: Vec<String> = self.values.iter().map(|v| v.to_string()).collect();
        write!(f, "[{}]", values_str.join(" "))
    }
}

impl Stack {
    pub fn new() -> Stack {
        Stack { values: Vec::new() }
    }

    pub fn push(&mut self, value: String) {
        self.values.push(value);
    }

    pub fn pop(&mut self) -> Option<String> {
        self.values.pop()
    }

    pub fn peek(&self) -> Option<&String> {
        self.values.last()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }
}
