use crate::stack::Stack;

pub struct Function {
    start: usize,
    stack: Stack, 
}

impl Function {
    pub fn new() -> Function {
        Function {
            start: 0,
            stack: Stack::new(),
        }
    }
}
