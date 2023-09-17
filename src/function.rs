use crate::stack::{StackHistory, Stack};
use crate::parser::{parse_line};
use crate::token::{Token, TokenType};

const COMMENT_START: &str = "//";

pub struct Function {
    pub start: usize,
    pub takes: i32,
    pub body: String,
    pub stack_history: StackHistory,
}

impl Function {
    pub fn new() -> Function {
        Function {
            start: 0,
            takes: 0,
            body: String::new(),
            stack_history: StackHistory::new(),
        }
    }

    pub fn longest_line(&self) -> usize {
        self.body.lines().map(|line| line.len()).max().unwrap_or(0)
    }

    pub fn gen_stack_history(&mut self) {
        let mut stack_history = StackHistory::new();
        let mut stack = Stack::new();
        for line in self.body.lines() {
            if !line.trim().starts_with(COMMENT_START) { // if not comment
                let tokens = parse_line(line);
                for mut token in tokens {
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
                        // IMPORTANT: We need to set the operands before updating the stack.
                        stack.set_operands(&mut token);
                        stack.update(token);
                    }
                }
            } 
            stack_history.push(stack.clone());
        }
        for stack in stack_history.stacks.iter() {
            println!("");
            println!("{:?}", stack);
        }
        self.stack_history = stack_history
    }
}
