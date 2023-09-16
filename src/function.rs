use crate::stack::Stack;

pub struct Function {
    pub start: usize,
    pub body: String,
    pub stack: Stack,
    pub longest_line: usize,
    pub takes: usize,
}

impl Function {
    pub fn new(start: usize, body: String, stack: Stack) -> Function {
        Function {
            start,
            body: body.clone(),
            stack,
            longest_line: Function::longest_line(&body),
            takes: 0, 
        }
    }

    fn longest_line(body: &str) -> usize {
        body.lines().map(|line| line.len()).max().unwrap_or(0)
    }
}
