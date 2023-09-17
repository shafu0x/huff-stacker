use crate::stack::{StackHistory, Stack};
use crate::parser::{parse_line};
use crate::token::{Token, TokenType};

pub struct Function {
    pub start: usize,
    pub body: String,
    pub stack_history: StackHistory,
    pub takes: i32,
}

impl Function {
    pub fn new() -> Function {
        Function {
            start: 0,
            body: String::new(),
            stack_history: StackHistory::new(),
            takes: 0,
        }
    }

    pub fn longest_line(&self) -> usize {
        self.body.lines().map(|line| line.len()).max().unwrap_or(0)
    }

    pub fn gen_stack_history(&mut self) {
        let mut stack_history = StackHistory::new();
        let mut stack = Stack::new();
        for line in self.body.lines() {
            let tokens = parse_line(line);
            for token in tokens {
                if token.token_type == TokenType::Constant {
                    stack.push(token);
                } 
                else if token.token_type == TokenType::Reference {
                    stack.push(token);
                } 
                else if token.token_type == TokenType::Variable {
                    stack.push(token);
                } 
                else if token.token_type == TokenType::Takes_Placeholder {
                    stack.push_takes(self.takes);
                }  
                else if token.token_type == TokenType::Opcode {
                    stack.update(token);
                }
            }
            stack_history.push(stack.clone());
        }
        for stack in stack_history.stacks.iter() {
            println!("{:?}", stack);
        }
        self.stack_history = stack_history
    }
}
