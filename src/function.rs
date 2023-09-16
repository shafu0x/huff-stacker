use crate::stack::Stack;

pub struct Function {
    pub start: usize,
    pub body: String,
    pub stack: Stack,
    pub takes: i32,
}

impl Function {
    pub fn new() -> Function {
        Function {
            start: 0,
            body: String::new(),
            stack: Stack::new(),
            takes: 0,
        }
    }

    pub fn longest_line(&self) -> usize {
        self.body.lines().map(|line| line.len()).max().unwrap_or(0)
    }
}
