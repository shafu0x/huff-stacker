use crate::opcodes::Opcode;
use crate::function::Function;
use crate::parser::{parse_line};
use crate::token::{Token, TokenType};

const COMMENT_START: &str = "//";
const CONSTANT_START: &str = "0x";
const REFERENCE_START: &str = "[";
const VARIABLE_START: &str = "<";

// We insert this placeholder into a function if it takes more than 0 arguments.
// The stack usese this placeholder to determine where to insert the arguments.
pub const TAKES_PLACEHOLDER: &str = "$takes$";

pub fn generate_stack(function: &mut Function) -> Vec<LocalStack> {
    let mut stack = Vec::<LocalStack>::new();
    for line in function.body.lines() {
        let tokens = parse_line(line);
        let mut local_stack = LocalStack::new();
        for token in tokens {
            if token.token_type == TokenType::Constant {
                local_stack.push(token);
            } 
            else if token.token_type == TokenType::Reference {
                local_stack.push(token);
            } 
            else if token.token_type == TokenType::Variable {
                local_stack.push(token);
            } 
            else if token.token_type == TokenType::Takes_Placeholder {
                local_stack.push_takes(function.takes);
            }  
        }
        stack.push(local_stack);
    }
    stack
}

#[derive(Debug)]
pub struct Stack {
    pub values: Vec<Vec<String>>,
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

#[derive(Debug)]
pub struct LocalStack {
    pub values: Vec<Token>,
}

impl LocalStack {
    pub fn new() -> LocalStack {
        LocalStack { values: Vec::new() }
    }

    pub fn push(&mut self, value: Token) {
        self.values.push(value);
    }

    pub fn pop(&mut self) -> Option<Token> {
        self.values.pop()
    }

    pub fn peek(&self) -> Option<&Token> {
        self.values.last()
    }

    pub fn push_takes(&mut self, takes: i32) {
        if takes > 0 {
            for i in 0..takes {
                let mut token = Token::new();
                token.token_type = TokenType::Constant;
                self.values.push(token);
            }
        }
    }
}
