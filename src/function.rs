use crate::stack::Stack;

pub struct Function {
    pub start: usize,
    pub body: String,
    pub stack: Stack,
    pub longest_line: usize,
}

impl Function {
    pub fn new(start: usize, body: String, stack: Stack, longest_line: usize) -> Function {
        Function {
            start,
            body,
            stack,
            longest_line,
        }
    }
}
