use crate::stack::Stack;

pub struct Function {
    start: usize,
    body: String,
    stack: Stack, 
}

impl Function {
    pub fn new(start: usize, body: String, stack: Stack) -> Function {
        Function {
            start, body, stack
        }
    }
}
