use crate::opcodes::Opcode;
use crate::function::Function;

const COMMENT_START: &str = "//";
const CONSTANT_START: &str = "0x";
const REFERENCE_START: &str = "[";
const VARIABLE_START: &str = "<";

// We insert this placeholder into a function if it takes more than 0 arguments.
// The stack usese this placeholder to determine where to insert the arguments.
pub const TAKES_PLACEHOLDER: &str = "$takes$";

#[derive(Debug)]
pub struct Stack {
    pub values: Vec<Vec<String>>,
}

pub fn generate_stack(function: &mut Function) -> Stack {
    let mut stack = Stack::new();
    for line in function.body.lines() {
        match line.trim() {
            line if line.starts_with(TAKES_PLACEHOLDER) => stack.push_takes(function.takes),
            line if line.starts_with(CONSTANT_START) => stack.push(line.to_string()),
            line if line.starts_with(REFERENCE_START) => stack.push(line.to_lowercase()),
            line if line.starts_with(VARIABLE_START) => stack.push(line.to_string()),
            line if line.starts_with(COMMENT_START) => stack.dup_last(),
            _ => stack.update(Opcode::from_string(line.trim())),
        }
    }
    stack
}

impl Stack {
    pub fn new() -> Stack {
        Stack { values: Vec::new() }
    }

    pub fn update(&mut self, opcode: Opcode) {
        if opcode.pops == 1 && opcode.pushes == 1 {
            let popped = self.peek().unwrap().clone();
            let result = format!("{}: {}", opcode.name, popped);
            self.pop_and_push(result);
            return;
        }
        if opcode.pops > 0 {
            self.pop(opcode.pops);
        }
        if opcode.pops == 0 && opcode.pushes == 0 {
            self.dup_last();
        }
    }

    pub fn dup_last(&mut self) {
        let last_values = self.values.last().unwrap().clone();
        self.values.push(last_values);
    }

    pub fn push(&mut self, value: String) {
        if self.values.len() == 0 {
            self.values.push(vec![value]);
        } else {
            let mut last_values = self.values.last().unwrap().clone();
            last_values.push(value);
            self.values.push(last_values);
        }
    }

    pub fn push_takes(&mut self, takes: i32) {
        if takes > 0 {
            let mut takes_vec = Vec::new();
            for i in 0..takes {
                takes_vec.push(format!("a{}", i));
            }
            self.values.push(takes_vec);
        }
    }

    pub fn pop(&mut self, pops: usize) {
        if self.values.len() == 0 {
            return;
        }

        let mut last_values = self.values.last().unwrap().clone();
        for _ in 0..pops {
            last_values.pop();
        }
        self.values.push(last_values);
    }

    pub fn pop_and_push(&mut self, value: String) {
        if self.values.len() == 0 {
            return;
        }
        let mut last_values = self.values.last().unwrap().clone();
        last_values.pop();
        last_values.push(value);
        self.values.push(last_values);
    }

    pub fn peek(&self) -> Option<&String> {
        self.values[self.values.len() - 1].last()
    }
}
